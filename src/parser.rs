// parser: converts token stream into abstract syntax tree
// performs strict syntax validation and builds ast nodes

use crate::ast::{Block, Expression, Position, Program, Statement};
use crate::types::{Token, TokenKind};

/// parser state for building ast from tokens
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// creates a new parser from a token stream
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// returns current token without consuming it
    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// looks ahead n tokens without consuming
    fn peek(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.position + n)
    }

    /// advances to the next token
    fn advance(&mut self) {
        if self.position < self.tokens.len() {
            self.position += 1;
        }
    }

    /// reports a parse error with position information and panics
    fn error(&self, msg: &str) -> ! {
        if let Some(token) = self.current() {
            panic!(
                "Parse error at line {}, column {}: {}",
                token.line, token.column, msg
            );
        } else {
            panic!("Parse error: {} (at end of file)", msg);
        }
    }

    /// expects a specific keyword and advances, errors if not found
    fn expect(&mut self, expected: &str) {
        if let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Keyword(k) if k == expected => {
                    self.advance();
                }
                _ => {
                    self.error(&format!(
                        "Expected '{}', but found {:?}",
                        expected, token.kind
                    ));
                }
            }
        } else {
            self.error(&format!("Expected '{}', but reached end of file", expected));
        }
    }

    /// parses the entire program starting with hai and ending with kthxbye
    pub fn parse_program(&mut self) -> Program {
        // lolcode programs must start with hai
        self.expect("HAI");

        // read optional version number after hai (defaults to 1.2)
        let version = if let Some(token) = self.current() {
            if let TokenKind::Number(n) = &token.kind {
                let v = n.clone();
                self.advance();
                v
            } else {
                "1.2".to_string()
            }
        } else {
            "1.2".to_string()
        };

        let mut statements = Vec::new();

        // parse statements until we find kthxbye
        while let Some(token) = self.current() {
            match &token.kind {
                TokenKind::Keyword(k) if k == "KTHXBYE" => {
                    self.advance();
                    return Program {
                        version,
                        body: Block { statements },
                    };
                }
                TokenKind::Newline | TokenKind::Comment(_) => {
                    self.advance();
                }
                _ => {
                    if let Some(stmt) = self.parse_statement() {
                        statements.push(stmt);
                    }
                }
            }
        }

        // reached end without kthxbye
        self.error("Expected 'KTHXBYE' at end of program");
    }

    /// parses a block of statements until an end keyword is reached
    fn parse_block(&mut self, end_keywords: &[&str]) -> Block {
        let mut statements = Vec::new();

        while let Some(token) = self.current() {
            // check if we've reached a block terminator
            if let TokenKind::Keyword(k) = &token.kind {
                // special handling for "no wai" (two-word keyword)
                if k == "NO" && end_keywords.contains(&"NO") {
                    if let Some(next) = self.peek(1) {
                        if matches!(&next.kind, TokenKind::Keyword(w) if w == "WAI") {
                            break;
                        }
                    }
                } else if end_keywords.contains(&k.as_str()) {
                    break;
                }
            }

            match &token.kind {
                TokenKind::Newline | TokenKind::Comment(_) => {
                    self.advance();
                }
                _ => {
                    if let Some(stmt) = self.parse_statement() {
                        statements.push(stmt);
                    }
                }
            }
        }

        Block { statements }
    }

    /// parses a single statement (declaration, assignment, visible, control flow)
    fn parse_statement(&mut self) -> Option<Statement> {
        let pos = self.current().map(|t| Position {
            line: t.line,
            column: t.column,
        })?;
        // check for block structures first (conditionals and loops)
        if let Some(token) = self.current() {
            if let TokenKind::Keyword(k) = &token.kind {
                // o rly? - conditional with ya rly and optional no wai
                if k == "O" {
                    if let Some(next) = self.peek(1) {
                        if matches!(&next.kind, TokenKind::Keyword(k) if k == "RLY?") {
                            self.advance();
                            self.advance();

                            // skip newlines between o rly? and ya rly
                            while matches!(
                                self.current().map(|t| &t.kind),
                                Some(TokenKind::Newline)
                            ) {
                                self.advance();
                            }

                            // expect ya rly block start
                            if let Some(t) = self.current() {
                                if let TokenKind::Keyword(k) = &t.kind {
                                    if k == "YA" {
                                        if let Some(next) = self.peek(1) {
                                            if matches!(&next.kind, TokenKind::Keyword(k) if k == "RLY")
                                            {
                                                self.advance();
                                                self.advance();
                                            } else {
                                                self.error("Expected RLY after YA");
                                            }
                                        }
                                    } else {
                                        self.error("Expected YA RLY after O RLY?");
                                    }
                                }
                            }

                            // parse the ya rly block body
                            let ya_rly = self.parse_block(&["NO", "OIC", "MEBBE"]);

                            // check for optional no wai (else) block
                            let no_wai = if let Some(t) = self.current() {
                                if let TokenKind::Keyword(k) = &t.kind {
                                    if k == "NO" {
                                        if let Some(next) = self.peek(1) {
                                            if matches!(&next.kind, TokenKind::Keyword(k) if k == "WAI")
                                            {
                                                self.advance();
                                                self.advance();
                                                Some(self.parse_block(&["OIC"]))
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            } else {
                                None
                            };

                            // expect oic to close the conditional
                            self.expect("OIC");

                            return Some(Statement::ORly {
                                ya_rly,
                                no_wai,
                                pos,
                            });
                        }
                    }
                }

                // im in yr loop - parse entire loop construct
                if k == "IM" {
                    if let Some(t1) = self.peek(1) {
                        if matches!(&t1.kind, TokenKind::Keyword(k) if k == "IN") {
                            if let Some(t2) = self.peek(2) {
                                if matches!(&t2.kind, TokenKind::Keyword(k) if k == "YR") {
                                    if let Some(t3) = self.peek(3) {
                                        if matches!(&t3.kind, TokenKind::Keyword(k) if k == "LOOP")
                                        {
                                            self.advance();
                                            self.advance();
                                            self.advance();
                                            self.advance();

                                            // parse loop body with special handling for im outta yr loop
                                            let body = self.parse_loop_body();

                                            return Some(Statement::Loop { body, pos });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // parse individual statement types
        if let Some(token) = self.current() {
            match &token.kind {
                // visible - output statement
                TokenKind::Keyword(k) if k == "VISIBLE" => {
                    self.advance();
                    let mut expressions = Vec::new();

                    // parse first expression if present
                    if let Some(t) = self.current() {
                        if !matches!(&t.kind, TokenKind::Newline) {
                            expressions.push(self.parse_expression());
                        }
                    }

                    // parse additional expressions for multi-value output
                    // visible can print multiple values separated by spaces
                    loop {
                        match self.current() {
                            None
                            | Some(Token {
                                kind: TokenKind::Newline,
                                ..
                            }) => break,
                            Some(Token {
                                kind: TokenKind::Identifier(_),
                                ..
                            })
                            | Some(Token {
                                kind: TokenKind::Number(_),
                                ..
                            })
                            | Some(Token {
                                kind: TokenKind::StringLiteral(_),
                                ..
                            })
                            | Some(Token {
                                kind: TokenKind::Keyword(_),
                                ..
                            }) => {
                                expressions.push(self.parse_expression());
                            }
                            Some(t) => {
                                self.error(&format!(
                                    "Unexpected token after expression in VISIBLE: {:?}",
                                    t.kind
                                ));
                            }
                        }
                    }

                    return Some(Statement::Visible { expressions, pos });
                }

                // i has a - variable declaration
                TokenKind::Keyword(k) if k == "I" => {
                    self.advance();
                    self.expect("HAS");
                    self.expect("A");

                    // read variable identifier
                    let name = if let Some(t) = self.current() {
                        if let TokenKind::Identifier(id) = &t.kind {
                            let n = id.clone();
                            self.advance();
                            n
                        } else {
                            self.error("Expected identifier after I HAS A");
                        }
                    } else {
                        self.error("Expected identifier after I HAS A");
                    };

                    // check for optional itz initialization
                    let value = if let Some(t) = self.current() {
                        if matches!(&t.kind, TokenKind::Keyword(k) if k == "ITZ") {
                            self.advance();
                            let expr = self.parse_expression();

                            // ensure expression ends at newline or eof
                            if let Some(next) = self.current() {
                                if !matches!(&next.kind, TokenKind::Newline) {
                                    self.error(&format!(
                                        "Expected newline after expression, found {:?}",
                                        next.kind
                                    ));
                                }
                            }

                            Some(expr)
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    return Some(Statement::Declaration { name, value, pos });
                }

                // assignment: <identifier> r <value>
                TokenKind::Identifier(id) => {
                    let name = id.clone();
                    self.advance();

                    if let Some(t) = self.current() {
                        if matches!(&t.kind, TokenKind::Keyword(k) if k == "R") {
                            self.advance();
                            let expr = self.parse_expression();

                            // ensure expression ends at newline or eof
                            if let Some(next) = self.current() {
                                if !matches!(&next.kind, TokenKind::Newline) {
                                    self.error(&format!(
                                        "Expected newline after expression, found {:?}",
                                        next.kind
                                    ));
                                }
                            }

                            let value = Some(expr);
                            return Some(Statement::Assignment { name, value, pos });
                        }
                    }

                    None
                }

                // expression statements that set the implicit it variable
                // e.g., both saem x an y, sum of a an b, etc.
                TokenKind::Keyword(k)
                    if matches!(
                        k.as_str(),
                        "SUM" | "DIFF" | "PRODUKT" | "QUOSHUNT" | "MOD" | "BOTH" | "DIFFRINT"
                    ) =>
                {
                    let expr = self.parse_expression();

                    // ensure expression ends at newline or eof
                    if let Some(next) = self.current() {
                        if !matches!(&next.kind, TokenKind::Newline) {
                            self.error(&format!(
                                "Expected newline after expression statement, found {:?}",
                                next.kind
                            ));
                        }
                    }

                    return Some(Statement::ExpressionStatement {
                        expression: expr,
                        pos,
                    });
                }

                // skip unknown tokens
                _ => {
                    self.advance();
                    None
                }
            }
        } else {
            None
        }
    }

    /// parses loop body until im outta yr loop is found
    fn parse_loop_body(&mut self) -> Block {
        let mut statements = Vec::new();

        while let Some(token) = self.current() {
            // check for loop terminator: im outta yr loop
            if let TokenKind::Keyword(k) = &token.kind {
                if k == "IM" {
                    if let Some(t1) = self.peek(1) {
                        if matches!(&t1.kind, TokenKind::Identifier(id) if id == "OUTTA") {
                            if let Some(t2) = self.peek(2) {
                                if matches!(&t2.kind, TokenKind::Keyword(k) if k == "YR") {
                                    if let Some(t3) = self.peek(3) {
                                        if matches!(&t3.kind, TokenKind::Keyword(k) if k == "LOOP")
                                        {
                                            self.advance();
                                            self.advance();
                                            self.advance();
                                            self.advance();
                                            return Block { statements };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match &token.kind {
                TokenKind::Newline | TokenKind::Comment(_) => {
                    self.advance();
                }
                _ => {
                    if let Some(stmt) = self.parse_statement() {
                        statements.push(stmt);
                    }
                }
            }
        }

        Block { statements }
    }

    /// parses an expression (literal, identifier, or operation)
    fn parse_expression(&mut self) -> Expression {
        let token = self.current().expect("unexpected EOF in expression");
        let pos = Position {
            line: token.line,
            column: token.column,
        };

        match &token.kind {
            // number literal
            TokenKind::Number(n) => {
                let num = n.clone();
                self.advance();
                Expression::Number(num, pos)
            }

            // string literal
            TokenKind::StringLiteral(s) => {
                let string = s.clone();
                self.advance();
                Expression::String(string, pos)
            }

            // identifier
            TokenKind::Identifier(id) => {
                let ident = id.clone();
                self.advance();
                Expression::Identifier(ident, pos)
            }

            // sum of - addition
            TokenKind::Keyword(k) if k == "SUM" => {
                self.advance();
                self.expect("OF");
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::Sum {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            // diff of - subtraction
            TokenKind::Keyword(k) if k == "DIFF" => {
                self.advance();
                self.expect("OF");
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::Diff {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            // produkt of - multiplication
            TokenKind::Keyword(k) if k == "PRODUKT" => {
                self.advance();
                self.expect("OF");
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::Produkt {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            // quoshunt of - division
            TokenKind::Keyword(k) if k == "QUOSHUNT" => {
                self.advance();
                self.expect("OF");
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::Quoshunt {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            // mod of - modulo
            TokenKind::Keyword(k) if k == "MOD" => {
                self.advance();
                self.expect("OF");
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::Mod {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            // both saem - equality comparison
            TokenKind::Keyword(k) if k == "BOTH" => {
                self.advance();
                self.expect("SAEM");
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::BothSaem {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            // diffrint - inequality comparison
            TokenKind::Keyword(k) if k == "DIFFRINT" => {
                self.advance();
                let left = self.parse_expression();
                self.expect("AN");
                let right = self.parse_expression();
                Expression::Diffrint {
                    left: Box::new(left),
                    right: Box::new(right),
                    pos,
                }
            }

            _ => self.error(&format!("Unexpected token in expression: {:?}", token.kind)),
        }
    }
}
