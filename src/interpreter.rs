use std::{cell::RefCell, process::Command, rc::Rc};

use crate::{
    ast::*,
    types::{Type, Value},
};

// use crate::parse::{AstNode, CommandPart, InfixVerb};

#[derive(Debug, Clone)]
pub struct Env {
    vars: Vec<(String, Type, Value)>,
}

// #[derive(Debug, Clone)]
// pub enum Val {
//     Integer(i64),
//     Unit,
// }

fn envlookup(env: &Env, var: &str) -> Option<Value> {
    env.vars
        .iter()
        .find(|(s, _, _)| (var == s))
        .map(|(_, _, v)| v.clone())
}

pub fn eval(exp: &AstNode) -> (Value, Env) {
    eval_env(exp, Env { vars: vec![] })
}

pub fn eval_env(exp: &AstNode, env: Env) -> (Value, Env) {
    match exp {
        AstNode::InfixExpr { verb, lhs, rhs } => {
            let (v1, _) = eval_env(lhs, env.clone());
            let (v2, _) = eval_env(rhs, env.clone());
            let out = match verb {
                InfixVerb::Plus => match (v1, v2) {
                    (Value::Int(x1), Value::Int(x2)) => Value::Int(x1 + x2),
                    _ => todo!(),
                },
                InfixVerb::Minus => todo!(),
                InfixVerb::Times => todo!(),
                InfixVerb::Divide => todo!(),
            };
            (out, env)
        }
        AstNode::Integer(x) => (Value::Int(*x), env),
        AstNode::Ident(var) => (envlookup(&env, var).expect("identifier not found"), env),
        AstNode::Command { parts } => {
            let mut cmdline: String = String::new();
            for part in parts {
                let append = match part {
                    CommandPart::Text(s) => s.clone(),
                    CommandPart::Expr(e) => {
                        let (val, _) = eval_env(&e, env.clone());
                        match val {
                            Value::Int(x) => x.to_string(),
                            Value::Str(s) => s,
                            Value::Bool(_) => todo!(),
                            Value::List(_) => {
                                let Value::Str(s) = val.convert(&Type::Str).unwrap() else {
                                    todo!()
                                };
                                s
                            }
                            Value::Unit => String::new(),
                            Value::Command { command: _ } => todo!(),
                        }
                    }
                };
                cmdline.push_str(&append);
            }

            let args: Vec<_> = cmdline.trim().split_whitespace().collect();
            // println!("command: {:?}", args);

            let Some((program, args)) = args.split_first() else {
                todo!();
            };
            let mut cmd = Command::new(program);
            cmd.args(args);
            // let output = cmd.output();
            // println!("{:?}", output);

            // TODO: a "command" type which gets coerced easily.
            // for now, just interpret each command as an int
            // let x: String = String::from_utf8(output.unwrap().stdout).unwrap();

            // (Value::Str(x), env)
            (
                Value::Command {
                    command: Rc::new(RefCell::new(cmd)),
                },
                env,
            )
        }
        AstNode::Block(es) => {
            let mut out = Value::Unit;
            let mut block_env = env.clone();
            for e in es {
                (out, block_env) = eval_env(e, block_env);
                if let Value::Command { command } = &out {
                    let _ = command.borrow_mut().spawn().unwrap().wait();
                }
            }
            // if env.vars.is_empty() {
            // println!("block: {:?}", block_env);
            // }
            (out, env)
        }
        AstNode::Assign { ident, expr } => {
            if envlookup(&env, ident).is_none() {
                todo!()
            }
            let (new_v, _) = eval_env(expr, env.clone());

            let mut env = env;
            for (s, _t, v) in env.vars.iter_mut() {
                if ident == s {
                    *v = new_v;
                    break;
                }
            }
            (Value::Unit, env)
        }
        AstNode::Type(_) => todo!(),
        AstNode::Binding { ident, ty, expr } => {
            let (v, _) = eval_env(expr, env.clone());

            let Some(v) = v.convert(ty) else {
                todo!();
            };
            let mut env = env;
            env.vars
                .insert(0, (ident.to_string(), ty.clone(), v.clone()));
            (Value::Unit, env)
        }
    }
}
