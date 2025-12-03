// lexer: tokenizes lolcode source into a stream of tokens
// handles comments (btw and obtw...tldr) and tracks position information

use crate::types::{Token, TokenKind};

/// lexer state for tokenizing lolcode source
pub struct Lexer {
    pub source: String,
    pub position: usize,
    pub line: usize,
    pub column: usize,
}

impl Lexer {
    /// creates a new lexer from source string
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// tokenizes the entire source into a vector of tokens
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                '\n' => {
                    self.advance();
                    let line = self.line;
                    let column = self.column;
                    tokens.push(Token::new(TokenKind::Newline, line, column));
                }
                '"' => {
                    tokens.push(self.read_string());
                }
                _ if ch.is_whitespace() => {
                    self.advance();
                }
                _ if ch.is_ascii_digit() => {
                    tokens.push(self.read_number());
                }
                _ if ch.is_ascii_alphabetic() => {
                    tokens.push(self.read_word());
                }
                _ => {
                    // ignore unknown characters
                    self.advance();
                }
            }
        }

        tokens
    }

    /// peeks at the current character without consuming it
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    /// advances position by one character, updating line and column tracking
    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.position += 1;
    }

    /// reads a string literal enclosed in double quotes
    fn read_string(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut result = String::new();
        self.advance(); // skip opening quote

        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.advance(); // skip closing quote
                break;
            }
            result.push(ch);
            self.advance();
        }

        Token::new(TokenKind::StringLiteral(result), line, column)
    }

    /// reads a number literal (integer or float)
    fn read_number(&mut self) -> Token {
        let line = self.line;
        let column = self.column;
        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        Token::new(TokenKind::Number(result), line, column)
    }

    /// reads a word (keyword or identifier), checking for comments first
    fn read_word(&mut self) -> Token {
        let line = self.line;
        let column = self.column;

        // check for btw single-line comment before reading word
        if self.peek_word_matches("BTW") {
            for _ in 0..3 {
                self.advance();
            }
            return self.read_comment(line, column);
        }

        // check for obtw multiline comment before reading word
        if self.peek_word_matches("OBTW") {
            for _ in 0..4 {
                self.advance();
            }
            return self.read_multiline_comment(line, column);
        }

        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_ascii_alphanumeric() || ch == '?' {
                result.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        // distinguish between keywords and identifiers
        let kind = if Token::is_keyword(&result) {
            TokenKind::Keyword(result)
        } else {
            TokenKind::Identifier(result)
        };

        Token::new(kind, line, column)
    }

    /// checks if the next characters match a specific word (case-insensitive)
    fn peek_word_matches(&self, word: &str) -> bool {
        let mut pos = self.position;
        for expected_ch in word.chars() {
            if let Some(ch) = self.source.chars().nth(pos) {
                if ch.to_ascii_uppercase() != expected_ch {
                    return false;
                }
                pos += 1;
            } else {
                return false;
            }
        }
        // ensure word boundary (not part of a longer word)
        if let Some(ch) = self.source.chars().nth(pos) {
            !ch.is_ascii_alphanumeric()
        } else {
            true
        }
    }

    /// reads a single-line comment (btw) until end of line
    fn read_comment(&mut self, line: usize, column: usize) -> Token {
        let mut result = String::new();

        while let Some(ch) = self.peek() {
            if ch == '\n' {
                break;
            }
            result.push(ch);
            self.advance();
        }

        Token::new(TokenKind::Comment(result), line, column)
    }

    /// reads a multiline comment (obtw...tldr) with proper position tracking
    fn read_multiline_comment(&mut self, line: usize, column: usize) -> Token {
        let mut content = String::new();

        loop {
            // check if we reached end of source
            if self.position >= self.source.len() {
                break; // no tldr found, end comment
            }

            // check for tldr closing marker
            let word = self.peek_word_uppercase();

            if word == "TLDR" {
                // consume tldr marker with manual position tracking
                for _ in 0..4 {
                    if let Some(ch) = self.peek() {
                        if ch == '\n' {
                            self.line += 1;
                            self.column = 1;
                        } else {
                            self.column += 1;
                        }
                        self.position += 1;
                    }
                }
                break;
            }

            // accumulate comment content
            if let Some(ch) = self.peek() {
                content.push(ch);
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.position += 1;
            }
        }

        Token::new(TokenKind::Comment(content), line, column)
    }

    /// peeks ahead to read the next word in uppercase without advancing
    fn peek_word_uppercase(&self) -> String {
        let mut result = String::new();
        let mut pos = self.position;

        while let Some(ch) = self.source.chars().nth(pos) {
            if ch.is_ascii_alphabetic() {
                result.push(ch);
                pos += 1;
            } else {
                break;
            }
        }

        result.to_uppercase()
    }
}
