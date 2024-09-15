use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

use crate::{
    ast::*,
    types::{Type, Value},
};

#[derive(Debug, Clone)]
pub struct Env {
    vars: Vec<(String, Type, Value)>,
}

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
        AstNode::Boolean(b) => (Value::Bool(*b), env),
        AstNode::Ident(var) => (envlookup(&env, var).expect("identifier not found"), env),
        AstNode::Command(tokens) => {
            // evaluate each token down to a string with concatenated parts
            let mut args: Vec<String> = Vec::new();

            for tok in tokens {
                let parts: Vec<String> = tok
                    .0
                    .iter()
                    .map(|ast| {
                        let (val, _) = eval_env(&ast, env.clone());
                        let Some(Value::Str(s)) = val.convert(&Type::Str) else {
                            panic!()
                        };
                        s
                    })
                    .collect();
                args.push(parts.concat());
            }

            let Some((program, args)) = args.split_first() else {
                // commands have to have at least one token (i.e. the program)
                todo!();
            };
            let mut cmd = Command::new(program);
            cmd.args(args);

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
                todo!("failed to convert");
            };
            let mut env = env;
            env.vars
                .insert(0, (ident.to_string(), ty.clone(), v.clone()));
            (Value::Unit, env)
        }
        AstNode::QuoteString(qs) => {
            let parts: Vec<String> = qs
                .iter()
                .map(|ast| {
                    let (val, _) = eval_env(&ast, env.clone());
                    let Some(Value::Str(s)) = val.convert(&Type::Str) else {
                        panic!()
                    };
                    s
                })
                .collect();
            (Value::Str(parts.concat()), env)
        }
        AstNode::StringLiteral(s) => (Value::Str(String::from(s)), env),
        AstNode::Unit => todo!(),
        AstNode::IfThenElse {
            cond,
            t_block,
            f_block,
        } => {
            let (out, _) = eval_env(cond, env.clone());
            let Value::Bool(b) = out else {
                todo!();
            };
            let (out, _) = if b {
                eval_env(t_block, env.clone())
            } else {
                eval_env(f_block, env.clone())
            };
            (out, env)
        }
        AstNode::RecordValue(r) => {
            let mut out = HashMap::new();
            for (key, ast) in r.iter() {
                let (val, _) = eval_env(ast, env.clone());
                out.insert(key.clone(), val);
            }
            (Value::Record(out), env)
        }
    }
}
