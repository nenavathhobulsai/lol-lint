// linter: semantic analysis and code quality checks
// performs variable tracking, detects errors and warnings

use crate::ast::{Block, Expression, Program, Statement};
use std::collections::HashSet;

/// linter state tracking errors, warnings, and variable usage
#[derive(Debug)]
pub struct Linter {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    declared_vars: HashSet<String>,
    used_vars: HashSet<String>,
}

impl Linter {
    /// performs semantic analysis on the entire program
    pub fn lint(program: &Program) -> Self {
        let mut linter = Linter {
            errors: vec![],
            warnings: vec![],
            declared_vars: HashSet::new(),
            used_vars: HashSet::new(),
        };

        linter.check_block(&program.body);
        linter.check_unused_variables();

        linter
    }

    /// recursively checks all statements in a block
    fn check_block(&mut self, block: &Block) {
        for stmt in &block.statements {
            self.check_statement(stmt);
        }
    }

    /// validates a single statement and performs semantic checks
    fn check_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Declaration { name, value, pos } => {
                // detect double declarations
                if self.declared_vars.contains(name) {
                    self.errors.push(format!(
                        "error: variable '{}' declared twice (line {}, column {})",
                        name, pos.line, pos.column
                    ));
                } else {
                    self.declared_vars.insert(name.clone());
                }

                // check initialization expression if present
                if let Some(expr) = value {
                    self.check_expression(expr);
                }
            }

            Statement::Assignment { name, value, pos } => {
                // detect assignment to undeclared variables
                if !self.declared_vars.contains(name) {
                    self.errors.push(format!(
                        "error: assignment to undeclared variable '{}' (line {}, column {})",
                        name, pos.line, pos.column
                    ));
                } else {
                    self.used_vars.insert(name.clone());
                }

                // check assignment expression if present
                if let Some(expr) = value {
                    self.check_expression(expr);
                }
            }

            Statement::Visible {
                expressions,
                pos: _,
            } => {
                // validate all expressions in output statement
                for expr in expressions {
                    self.check_expression(expr);
                }
            }

            Statement::ORly {
                ya_rly,
                no_wai,
                pos,
            } => {
                // warn about empty if branches
                if ya_rly.statements.is_empty() {
                    self.warnings.push(format!(
                        "warning: YA RLY block is empty (line {}, column {})",
                        pos.line, pos.column
                    ));
                }

                self.check_block(ya_rly);

                // warn about missing else branches
                if let Some(no_block) = no_wai {
                    self.check_block(no_block);
                } else {
                    self.warnings.push(format!(
                        "warning: O RLY? without NO WAI branch (line {}, column {})",
                        pos.line, pos.column
                    ));
                }
            }

            Statement::Loop { body, pos } => {
                // warn about empty loop bodies
                if body.statements.is_empty() {
                    self.warnings.push(format!(
                        "warning: empty loop body (line {}, column {})",
                        pos.line, pos.column
                    ));
                }

                self.check_block(body);
            }

            Statement::ExpressionStatement { expression, pos: _ } => {
                self.check_expression(expression);

                // detect constant expressions that are always true/false
                if let Some(warning) = self.check_constant_expression(expression) {
                    self.warnings.push(warning);
                }
            }
        }
    }

    /// validates an expression and checks variable usage
    fn check_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(name, pos) => {
                // detect use of undeclared variables
                if !self.declared_vars.contains(name) {
                    self.errors.push(format!(
                        "error: use of undeclared variable '{}' (line {}, column {})",
                        name, pos.line, pos.column
                    ));
                } else {
                    self.used_vars.insert(name.clone());
                }
            }

            Expression::Number(_, _) | Expression::String(_, _) => {
                // literals are always valid
            }

            // recursively check binary operations
            Expression::Sum { left, right, .. }
            | Expression::Diff { left, right, .. }
            | Expression::Produkt { left, right, .. }
            | Expression::Quoshunt { left, right, .. }
            | Expression::Mod { left, right, .. }
            | Expression::BothSaem { left, right, .. }
            | Expression::Diffrint { left, right, .. } => {
                self.check_expression(left);
                self.check_expression(right);
            }
        }
    }

    /// detects constant expressions that always evaluate to true or false
    fn check_constant_expression(&self, expr: &Expression) -> Option<String> {
        match expr {
            Expression::BothSaem { left, right, pos } => {
                // detect comparisons between identical number literals
                if let (Expression::Number(n1, _), Expression::Number(n2, _)) =
                    (left.as_ref(), right.as_ref())
                {
                    if n1 == n2 {
                        return Some(format!(
                            "warning: BOTH SAEM {} AN {} is always true (line {}, column {})",
                            n1, n2, pos.line, pos.column
                        ));
                    } else {
                        return Some(format!(
                            "warning: BOTH SAEM {} AN {} is always false (line {}, column {})",
                            n1, n2, pos.line, pos.column
                        ));
                    }
                }

                // detect comparisons between identical string literals
                if let (Expression::String(s1, _), Expression::String(s2, _)) =
                    (left.as_ref(), right.as_ref())
                {
                    if s1 == s2 {
                        return Some(format!(
                            "warning: BOTH SAEM \"{}\" AN \"{}\" is always true (line {}, column {})",
                            s1, s2, pos.line, pos.column
                        ));
                    } else {
                        return Some(format!(
                            "warning: BOTH SAEM \"{}\" AN \"{}\" is always false (line {}, column {})",
                            s1, s2, pos.line, pos.column
                        ));
                    }
                }

                // detect comparisons between identical constants (win, fail)
                if let (Expression::Identifier(i1, _), Expression::Identifier(i2, _)) =
                    (left.as_ref(), right.as_ref())
                {
                    if i1 == i2 && (i1 == "WIN" || i1 == "FAIL") {
                        return Some(format!(
                            "warning: BOTH SAEM {} AN {} is always true (line {}, column {})",
                            i1, i2, pos.line, pos.column
                        ));
                    }
                }
            }

            Expression::Diffrint { left, right, pos } => {
                // detect diffrint comparisons between identical literals
                if let (Expression::Number(n1, _), Expression::Number(n2, _)) =
                    (left.as_ref(), right.as_ref())
                {
                    if n1 == n2 {
                        return Some(format!(
                            "warning: DIFFRINT {} AN {} is always false (line {}, column {})",
                            n1, n2, pos.line, pos.column
                        ));
                    } else {
                        return Some(format!(
                            "warning: DIFFRINT {} AN {} is always true (line {}, column {})",
                            n1, n2, pos.line, pos.column
                        ));
                    }
                }
            }

            _ => {}
        }

        None
    }

    /// warns about variables that are declared but never used
    fn check_unused_variables(&mut self) {
        for var in &self.declared_vars {
            if !self.used_vars.contains(var) {
                self.warnings.push(format!(
                    "warning: variable '{}' declared but never used",
                    var
                ));
            }
        }
    }

    /// returns true if any errors were detected during linting
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
