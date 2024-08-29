use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{ast::*, types::Type};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct KleyParser;

impl KleyParser {
    pub fn build_ast(code: &str) -> Result<AstNode, Error<Rule>> {
        let pairs = KleyParser::parse(Rule::program, code)?;

        // println!("{}", pairs);

        let mut ast = vec![];

        for pair in pairs {
            match pair.as_rule() {
                Rule::EOI => {}
                _ => {
                    ast.push(parse_term(pair));
                }
            }
        }

        Ok(AstNode::Block(ast))
    }
}

fn get_string(pairs: &mut Pairs<Rule>) -> String {
    pairs.next().unwrap().as_str().trim().to_owned()
}

fn get_ast(pairs: &mut Pairs<Rule>) -> Box<AstNode> {
    Box::new(parse_term(pairs.next().unwrap()))
}

fn parse_term(pair: Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::EOI => todo!(),
        Rule::WHITESPACE => todo!(),
        Rule::program => todo!(),
        Rule::assign => {
            let mut inner = pair.into_inner();
            // println!("assign: {}", inner);
            let ident = get_string(&mut inner);
            let expr = get_ast(&mut inner);
            AstNode::Assign { ident, expr }
        }
        Rule::binding => {
            let mut inner = pair.into_inner();
            // println!("binding: {}", inner);
            let ident = get_string(&mut inner);
            let ty = Type::parse(&get_string(&mut inner));
            let expr = get_ast(&mut inner);
            AstNode::Binding { ident, ty, expr }
        }
        Rule::expression => todo!(),
        Rule::expr => parse_term(pair.into_inner().next().unwrap()),
        Rule::atom => todo!(),
        Rule::block => AstNode::Block(pair.into_inner().map(parse_term).collect()),
        Rule::command_text => todo!(),
        Rule::command => AstNode::Command {
            parts: pair
                .into_inner()
                .map(|p| {
                    if p.as_rule() == Rule::command_text {
                        CommandPart::Text(p.as_str().to_owned())
                    } else {
                        CommandPart::Expr(parse_term(p))
                    }
                })
                .collect(),
        },
        Rule::number => {
            let istr = pair.as_str().trim();
            let num = istr.parse().unwrap();
            AstNode::Integer(num)
        }
        Rule::boolean => todo!(),
        Rule::ident => AstNode::Ident(String::from(pair.as_str())),
        Rule::infix => todo!(),
        Rule::infix_expr => {
            let mut inner = pair.into_inner();
            let lhs = get_ast(&mut inner);
            let vstr = get_string(&mut inner);
            let rhs = get_ast(&mut inner);

            AstNode::InfixExpr {
                verb: match vstr.as_str() {
                    "+" => InfixVerb::Plus,
                    "-" => InfixVerb::Minus,
                    "*" => InfixVerb::Times,
                    "/" => InfixVerb::Divide,
                    _ => panic!("unexpected infix verb: {}", vstr),
                },
                lhs,
                rhs,
            }
        }
        Rule::COMMENT => todo!(),
        Rule::newline => todo!(),
        Rule::stmt => todo!(),
        Rule::r#type => todo!(),
    }
}
