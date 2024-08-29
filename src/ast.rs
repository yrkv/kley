use crate::types::Type;

#[derive(Debug)]
pub enum AstNode {
    InfixExpr {
        verb: InfixVerb,
        lhs: Box<AstNode>,
        rhs: Box<AstNode>,
    },
    Integer(i64),
    Ident(String),
    Type(String),
    Command {
        parts: Vec<CommandPart>,
    },
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
}

#[derive(Debug)]
pub enum InfixVerb {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug)]
pub enum CommandPart {
    Text(String),
    Expr(AstNode),
}
