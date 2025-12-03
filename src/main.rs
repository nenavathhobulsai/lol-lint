// lol-lint: a strict linter for lolcode
// enforces syntax rules and performs semantic analysis

mod ast;
mod lexer;
mod linter;
mod parser;
mod types;

use clap::Parser as ClapParser;
use std::fs;
use std::process;
use lexer::Lexer;
use parser::Parser;
use linter::Linter;
use colored::*;
use serde::Serialize;

/// command-line interface structure for argument parsing
#[derive(ClapParser)]
#[command(name = "lol-lint")]
#[command(version = "0.1.0")]
#[command(about = "A linter for LOLCODE", long_about = None)]
struct Cli {
    /// input lolcode file to lint
    file: String,

    /// output results as json for ci/cd integration
    #[arg(long)]
    json: bool,

    /// show code statistics (loc, variables, loops, etc.)
    #[arg(long)]
    stats: bool,

    /// disable colored output for terminal compatibility
    #[arg(long)]
    no_color: bool,

    /// show debug information including tokens and ast
    #[arg(long)]
    debug: bool,
}

/// json output format for machine-readable results
#[derive(Serialize)]
struct JsonOutput {
    file: String,
    errors: Vec<String>,
    warnings: Vec<String>,
    stats: Option<Stats>,
}

/// code statistics collected during ast traversal
#[derive(Serialize)]
struct Stats {
    lines_of_code: usize,
    variables: usize,
    loops: usize,
    conditionals: usize,
    expressions: usize,
}

fn main() {
    let cli = Cli::parse();

    // disable colored output if requested for terminal compatibility
    if cli.no_color {
        colored::control::set_override(false);
    }

    // read source file into memory
    let content = match fs::read_to_string(&cli.file) {
        Ok(c) => c,
        Err(e) => {
            if !cli.json {
                eprintln!("{} Could not read file '{}': {}", "error:".red().bold(), cli.file, e);
            }
            process::exit(2);
        }
    };

    // tokenize the source code
    let mut lexer = Lexer::new(content.clone());
    let tokens = lexer.tokenize();

    // display tokens in debug mode
    if cli.debug {
        println!("{}", "--- Tokens ---".cyan().bold());
        for t in &tokens {
            println!("{:?}", t);
        }
        println!();
    }

    // parse tokens into abstract syntax tree
    let mut parser = Parser::new(tokens);
    let program = match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        parser.parse_program()
    })) {
        Ok(p) => p,
        Err(_) => {
            if !cli.json {
                eprintln!("{} Parsing failed", "error:".red().bold());
            }
            process::exit(2);
        }
    };

    // display ast in debug mode
    if cli.debug {
        println!("{}", "--- AST ---".cyan().bold());
        println!("{:#?}", program);
        println!();
    }

    // perform semantic analysis and linting
    let linter = Linter::lint(&program);

    // calculate code statistics if requested
    let stats = if cli.stats {
        Some(calculate_stats(&program, &content))
    } else {
        None
    };

    // format output based on requested mode
    if cli.json {
        let output = JsonOutput {
            file: cli.file,
            errors: linter.errors.clone(),
            warnings: linter.warnings.clone(),
            stats,
        };
        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    } else {
        print_human_readable(&linter, stats.as_ref());
    }

    // exit with appropriate code: 0 for success, 1 for lint errors
    if linter.has_errors() {
        process::exit(1);
    }
}

/// formats and prints linting results in human-readable format with colors
fn print_human_readable(linter: &Linter, stats: Option<&Stats>) {
    // display all errors in red
    for error in &linter.errors {
        println!("{}", error.red());
    }

    // display all warnings in yellow
    for warning in &linter.warnings {
        println!("{}", warning.yellow());
    }

    // print summary line with error and warning counts
    if !linter.errors.is_empty() || !linter.warnings.is_empty() {
        println!();
        let error_count = linter.errors.len();
        let warning_count = linter.warnings.len();
        
        let error_text = if error_count > 0 {
            format!("{} error{}", error_count, if error_count == 1 { "" } else { "s" }).red()
        } else {
            format!("{} errors", error_count).normal()
        };
        
        let warning_text = if warning_count > 0 {
            format!("{} warning{}", warning_count, if warning_count == 1 { "" } else { "s" }).yellow()
        } else {
            format!("{} warnings", warning_count).normal()
        };
        
        println!("{}, {}", error_text, warning_text);
    } else {
        println!("{} No linting issues found", "✓".green().bold());
    }

    // display statistics if available
    if let Some(s) = stats {
        println!();
        println!("{}", "--- Statistics ---".cyan().bold());
        println!("Lines of code:  {}", s.lines_of_code);
        println!("Variables:      {}", s.variables);
        println!("Loops:          {}", s.loops);
        println!("Conditionals:   {}", s.conditionals);
        println!("Expressions:    {}", s.expressions);
    }
}

/// calculates code statistics by analyzing the ast and source content
fn calculate_stats(program: &ast::Program, content: &str) -> Stats {
    // count non-empty, non-comment lines
    let lines_of_code = content.lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("BTW") && !trimmed.starts_with("OBTW")
        })
        .count();

    let mut variables = 0;
    let mut loops = 0;
    let mut conditionals = 0;
    let mut expressions = 0;

    count_in_block(&program.body, &mut variables, &mut loops, &mut conditionals, &mut expressions);

    Stats {
        lines_of_code,
        variables,
        loops,
        conditionals,
        expressions,
    }
}

/// recursively counts ast nodes in a block for statistics
fn count_in_block(block: &ast::Block, vars: &mut usize, loops: &mut usize, conds: &mut usize, exprs: &mut usize) {
    for stmt in &block.statements {
        match stmt {
            ast::Statement::Declaration { .. } => *vars += 1,
            ast::Statement::Loop { body, .. } => {
                *loops += 1;
                count_in_block(body, vars, loops, conds, exprs);
            }
            ast::Statement::ORly { ya_rly, no_wai, .. } => {
                *conds += 1;
                count_in_block(ya_rly, vars, loops, conds, exprs);
                if let Some(no_block) = no_wai {
                    count_in_block(no_block, vars, loops, conds, exprs);
                }
            }
            ast::Statement::Visible { expressions, .. } => {
                *exprs += expressions.len();
            }
            ast::Statement::Assignment { value, .. } => {
                if value.is_some() {
                    *exprs += 1;
                }
            }
            ast::Statement::ExpressionStatement { .. } => {
                *exprs += 1;
            }
        }
    }
}
