ol-lint

lol-lint is a strict, unapologetic linter for LOLCODE.
The language may be a joke. The linter is not.

Features

Strict syntax checking
If you write SUM OF 3 and forget the AN something, this tool will make sure you regret it.

Semantic analysis
Tracks variable declarations, detects use-before-declaration, double declarations, and other creative mistakes.

Code quality warnings
Unused variables, constant expressions, empty blocks, missing NO WAI branches.
If it smells, it gets flagged.

JSON output
CI/CD pipelines love JSON. Humans don't have to.

Statistics mode
For when you want detailed metrics about your masterpiece of chaos.

Debug mode
Dumps tokens and AST. Not pretty, but effective.

Installation
cargo build --release


The binary will appear at target/release/lol-lint.

Usage
# Basic linting
lol-lint file.lol

# Show statistics
lol-lint file.lol --stats

# JSON output
lol-lint file.lol --json

# Combine everything
lol-lint file.lol --json --stats --no-color

# Print tokens + AST
lol-lint file.lol --debug

Exit Codes

0 – No errors (warnings are tolerated)

1 – Linting errors found

2 – File not found or parser failure

Checks
Errors (fail the run)

Use of undeclared variables

Double declarations

Assigning to undeclared variables

Invalid or incomplete expressions

Malformed control flow structures

Warnings (won’t fail, but should hurt your pride)

Unused variables

Constant expressions (BOTH SAEM 5 AN 5 is not exactly “dynamic”)

Empty loop bodies

Empty YA RLY branches

Missing NO WAI in conditionals

Example Output
Clean file
$ lol-lint example.lol --stats
No linting issues found

--- Statistics ---
Lines of code:   17
Variables:       3
Loops:           1
Conditionals:    1
Expressions:     7

File with errors and warnings
$ lol-lint bad.lol
error: use of undeclared variable 'x' (line 4, column 9)
error: variable 'y' declared twice (line 6, column 1)
warning: variable 'unused' declared but never used
warning: BOTH SAEM 5 AN 5 is always true (line 8, column 1)

2 errors, 2 warnings

JSON output
$ lol-lint file.lol --json --stats
{
  "file": "file.lol",
  "errors": [],
  "warnings": [],
  "stats": {
    "lines_of_code": 17,
    "variables": 3,
    "loops": 1,
    "conditionals": 1,
    "expressions": 7
  }
}

CI/CD Integration
GitHub Actions example
- name: Lint LOLCODE
  run: |
    lol-lint src/main.lol --json > lint-results.json
    if [ $? -eq 1 ]; then
      echo "Linting failed"
      cat lint-results.json
      exit 1
    fi

Architecture Overview
┌──────────┐ → ┌────────┐ → ┌─────────┐ → ┌─────────┐
│  Lexer   │   │ Parser │   │   AST   │   │  Linter  │
└──────────┘   └────────┘   └─────────┘   └─────────┘
    Tokens       Syntax         Structure     Semantics


Lexer: Splits LOLCODE into tokens (including comment handling)

Parser: Validates syntax and constructs the AST

Linter: Runs semantic checks and code-quality analysis

License

MIT