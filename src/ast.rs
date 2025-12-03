// ast: abstract syntax tree definitions for lolcode
// represents the structure of a lolcode program after parsing

/// source code position for error reporting
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

/// expression nodes representing values and operations
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(String, Position),
    String(String, Position),
    Identifier(String, Position),

    // arithmetic operations
    Sum {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },
    Diff {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },
    Produkt {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },
    Quoshunt {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },
    Mod {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },

    // comparison operations
    BothSaem {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },
    Diffrint {
        left: Box<Expression>,
        right: Box<Expression>,
        pos: Position,
    },
}

/// statement nodes representing actions and control flow
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// variable declaration (i has a)
    Declaration {
        name: String,
        value: Option<Expression>,
        pos: Position,
    },

    /// variable assignment (x r value)
    Assignment {
        name: String,
        value: Option<Expression>,
        pos: Position,
    },

    /// output statement (visible)
    Visible {
        expressions: Vec<Expression>,
        pos: Position,
    },

    /// conditional statement (o rly? ya rly ... no wai ... oic)
    ORly {
        ya_rly: Block,
        no_wai: Option<Block>,
        pos: Position,
    },

    /// loop statement (im in yr loop ... im outta yr loop)
    Loop { body: Block, pos: Position },

    /// standalone expression statement (sets implicit it variable)
    Expr {
        expression: Expression,
        pos: Position,
    },
}

/// block of statements (used in control flow structures)
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statements: Vec<Statement>,
}

/// root program node containing version and body
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub version: String,
    pub body: Block,
}
