// types: token definitions for lolcode lexical analysis
// represents all token types with position tracking

/// represents different kinds of tokens in lolcode
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword(String),
    Identifier(String),
    Number(String),
    StringLiteral(String),
    Comment(String),
    Newline,
}

/// token with kind and position information for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// creates a new token with position information
    pub fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }

    /// checks if a word is a lolcode keyword
    pub fn is_keyword(word: &str) -> bool {
        matches!(
            word,
            "HAI" | "KTHXBYE" |
            "VISIBLE" | "GIMMEH" |
            "I" | "HAS" | "A" | "ITZ" | "R" | "AN" |
            "SUM" | "OF" | "DIFF" | "PRODUKT" | "QUOSHUNT" | "MOD" |
            "BOTH" | "SAEM" | "DIFFRINT" |
            "O" | "RLY?" | "YA" | "RLY" | "MEBBE" | "NO" | "WAI" | "OIC" |
            "IM" | "IN" | "YR" | "LOOP" | "UPPIN" | "NERFIN" | "TIL" | "WILE" |
            "HOW" | "DUZ" | "FOUND" | "MKAY" |
            "OBTW" | "TLDR"
        )
    }
}
