use std::collections::HashMap;

use pest::{
    error::Error,
    iterators::{Pair, Pairs},
    Parser,
};
use pest_derive::Parser;

use crate::{ast::*, types::Type, Args};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct KleyParser;

pub fn build_ast(pairs: Pairs<Rule>) -> Result<AstNode, Error<Rule>> {
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

pub fn display_pairs(pairs: &mut Pairs<Rule>, tab: usize) {
    for pair in pairs {
        println!(
            "{}{:?}: {:?}",
            "  ".repeat(tab),
            pair.as_rule(),
            pair.as_str()
        );
        display_pairs(&mut pair.into_inner(), tab + 1);
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
            // let ty = Type::parse(&get_string(&mut inner));
            let ty = Type::parse(inner.next().unwrap());
            let expr = get_ast(&mut inner);
            AstNode::Binding { ident, ty, expr }
        }
        Rule::expression => todo!(),
        Rule::expr => parse_term(pair.into_inner().next().unwrap()),
        Rule::atom => todo!(),
        // Rule::block => AstNode::Block(pair.into_inner().map(parse_term).collect()),
        // Rule::command_text => todo!(),
        // Rule::command => AstNode::Command {
        // parts: pair
        //     .into_inner()
        //     .map(|p| {
        //         if p.as_rule() == Rule::command_text {
        //             CommandPart::Text(p.as_str().to_owned())
        //         } else {
        //             CommandPart::Expr(parse_term(p))
        //         }
        //     })
        //     .collect(),
        // },
        Rule::number => {
            let istr = pair.as_str().trim();
            let num = istr.parse().unwrap();
            AstNode::Integer(num)
        }
        Rule::boolean => match pair.as_str().trim() {
            "true" => AstNode::Boolean(true),
            "false" => AstNode::Boolean(false),
            _ => todo!(),
        },
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
        Rule::command => {
            let mut tokens: Vec<CommandToken> = Vec::new();
            for command_token in pair.into_inner() {
                assert!(command_token.as_rule() == Rule::command_token);

                let mut token: Vec<AstNode> = Vec::new();
                for part in command_token.into_inner() {
                    match part.as_rule() {
                        Rule::command_text
                        | Rule::quote_string
                        | Rule::raw_string
                        | Rule::block_small => {
                            token.push(parse_term(part));
                        }
                        _ => unimplemented!(),
                    }
                }
                tokens.push(CommandToken(token));
            }

            AstNode::Command(tokens)
        }
        Rule::command_text => AstNode::StringLiteral(pair.as_str().to_string()),
        Rule::quote_string => {
            let mut out = Vec::new();
            for part in pair.into_inner() {
                match part.as_rule() {
                    Rule::quote_string_text | Rule::block_small => {
                        out.push(parse_term(part));
                    }
                    _ => unreachable!(),
                }
            }
            AstNode::QuoteString(out)
        }
        Rule::raw_string => parse_term(pair.into_inner().next().unwrap()),
        Rule::quote_string_text | Rule::raw_string_text => {
            AstNode::StringLiteral(pair.as_str().to_string())
        }
        Rule::r#type => todo!(),
        Rule::block_small => parse_term(pair.into_inner().next().unwrap()),
        Rule::block_large => {
            let mut out = Vec::new();
            for pair in pair.into_inner() {
                out.push(parse_term(pair));
            }
            AstNode::Block(out)
        }
        Rule::command_token => unreachable!(), // handled by Rule::command
        Rule::stmt => unreachable!(),
        Rule::COMMENT => unreachable!(),
        Rule::EOI => unreachable!(),
        Rule::WHITESPACE => unreachable!(),
        Rule::newline => unreachable!(),
        Rule::program => unreachable!(),
        Rule::call => {
            let mut inner = pair.into_inner();
            let name = get_string(&mut inner);
            let mut args = Vec::new();
            while inner.peek().is_some() {
                args.push(parse_term(inner.next().unwrap()));
            }
            AstNode::Call { name, args }
        }
        Rule::call_args => todo!(),
        Rule::ifthenelse => {
            let mut inner = pair.into_inner();
            let cond = get_ast(&mut inner);
            let t_block = get_ast(&mut inner);
            if inner.peek().is_none() {
                todo!("omitted else blocks not implemented yet");
            }
            let f_block = get_ast(&mut inner);
            AstNode::IfThenElse {
                cond,
                t_block,
                f_block,
            }
        }
        Rule::record_value => {
            let mut out = HashMap::new();
            let mut inner = pair.into_inner();
            while inner.peek().is_some() {
                let ident = get_string(&mut inner);
                let ast = get_ast(&mut inner);
                out.insert(ident, *ast);
            }
            AstNode::RecordValue(out)
        }
        Rule::t_str => todo!(),
        Rule::t_int => todo!(),
        Rule::t_bool => todo!(),
        Rule::t_unit => todo!(),
        Rule::t_float => todo!(),
        Rule::t_list => todo!(),
        Rule::t_map => todo!(),
        Rule::t_tuple => todo!(),
        Rule::t_variant => todo!(),
        Rule::t_record => todo!(),
        Rule::t_ident => todo!(),
        Rule::function_def => {
            let mut inner = pair.into_inner();
            let name = get_string(&mut inner);
            let mut args = Vec::new();
            let mut function_args = inner.next().unwrap().into_inner();
            while function_args.peek().is_some() {
                let ident = get_string(&mut function_args);
                let ty = Type::parse(function_args.next().unwrap());
                args.push((ident, ty));
            }
            let out = Type::parse(inner.next().unwrap());
            let block = get_ast(&mut inner);
            AstNode::Function {
                name,
                args,
                out,
                block,
            }
        }
        Rule::function_args => todo!(),
    }
}
