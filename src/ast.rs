use std::collections::HashMap;

use crate::types::Type;

#[derive(Debug)]
pub enum AstNode {
    Unit,
    InfixExpr {
        verb: InfixVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Integer(i64),
    Boolean(bool),
    Ident(String),
    // Type(String),
    Type(Type),
    /// QuoteString uses StringLiteral for quote_string_text, and any AstNode for the block_small
    /// Note that there's multiple empty strings:  QuoteString([]) and StringLiteral("")
    QuoteString(Vec<AstNode>),
    StringLiteral(String),
    Block(Vec<AstNode>),
    Binding {
        ident: String,
        ty: Type,
        expr: Box<AstNode>,
    },
    Assign {
        ident: String,
        expr: Box<AstNode>,
    },
    Command(Vec<CommandToken>),
    IfThenElse {
        cond: Box<AstNode>,
        t_block: Box<AstNode>,
        f_block: Box<AstNode>,
    },

    // RecordType {}
    RecordValue(HashMap<String, AstNode>),
}

#[derive(Debug)]
pub enum InfixVerb {
    Plus,
    Minus,
    Times,
    Divide,
}

/// Within each CommandToken, all the parts get concatenated together
/// each part is either actual text or an expression to evaluate into a Value::Str
#[derive(Debug)]
pub struct CommandToken(pub Vec<AstNode>);
