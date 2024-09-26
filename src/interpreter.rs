use std::{collections::HashMap, process::Command};

use crate::{ast::*, types::Type};

#[derive(Debug, Clone)]
pub struct Env {
    vars: Vec<(String, Type, Value)>,
    fns: HashMap<String, AstNode>,
}

fn envlookup(env: &Env, var: &str) -> Option<Value> {
    env.vars
        .iter()
        .find(|(s, _, _)| (var == s))
        .map(|(_, _, v)| v.clone())
}

pub fn eval(exp: &AstNode) -> Value {
    eval_env(
        exp,
        &Env {
            vars: Vec::new(),
            fns: HashMap::new(),
        },
    )
}

pub fn eval_env(exp: &AstNode, env: &Env) -> Value {
    match exp {
        AstNode::InfixExpr { verb, lhs, rhs } => {
            let v1 = eval_env(lhs, env);
            let v2 = eval_env(rhs, env);
            let out = match verb {
                InfixVerb::Plus => match (v1, v2) {
                    (Value::Int(x1), Value::Int(x2)) => Value::Int(x1 + x2),
                    _ => todo!(),
                },
                InfixVerb::Minus => todo!(),
                InfixVerb::Times => todo!(),
                InfixVerb::Divide => todo!(),
            };
            out
        }
        AstNode::Integer(x) => Value::Int(*x),
        AstNode::Boolean(b) => Value::Bool(*b),
        AstNode::Ident(var) => envlookup(&env, var).expect("identifier not found"),
        AstNode::Command(tokens) => {
            // evaluate each token down to a string with concatenated parts
            let mut args: Vec<String> = Vec::new();

            for tok in tokens {
                let parts: Vec<String> = tok
                    .0
                    .iter()
                    .map(|ast| {
                        let val = eval_env(&ast, env);
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
            // let mut cmd = Command::new(program);
            // cmd.args(args);

            let val_program = Value::Str(program.clone());
            let val_args = Value::List(args.iter().map(|a| Value::Str(a.clone())).collect());

            Value::Record(HashMap::from_iter([
                (String::from("program"), val_program),
                (String::from("args"), val_args),
                (String::from("_command"), Value::Unit),
                // TODO: stdin,stdout,stderr ???
            ]))
            // Value::Command {
            // command: Rc::new(RefCell::new(cmd)),
            // },
        }
        AstNode::Block(es) => {
            let mut out = Value::Unit;
            let mut block_env = env.clone();
            for e in es {
                // Match over the AstNode for constructs which alter the environment,
                // such as variable bindings, assignment, functions, or type aliases.
                // In the default case, it just evaluates the expression.
                out = match e {
                    AstNode::Binding { ident, ty, expr } => {
                        let v = eval_env(expr, &block_env);
                        let Some(v) = v.convert(ty) else {
                            todo!("failed to convert");
                        };
                        block_env
                            .vars
                            .insert(0, (ident.to_string(), ty.clone(), v.clone()));
                        Value::Unit
                    }
                    AstNode::Assign { ident, expr } => {
                        if envlookup(&block_env, ident).is_none() {
                            todo!()
                        }
                        let new_v = eval_env(expr, &block_env);
                        for (s, _t, v) in block_env.vars.iter_mut() {
                            if ident == s {
                                *v = new_v;
                                break;
                            }
                        }
                        Value::Unit
                    }
                    AstNode::Function { name, .. } => {
                        block_env.fns.insert(name.clone(), e.clone());
                        Value::Unit
                    }
                    _ => eval_env(e, &block_env),
                };

                // If the resulting value has some defined side effect (such as a command record)
                // then it should be acted upon here.
                if let Some((program, args)) = extract_command(&out) {
                    let _ = Command::new(program).args(args).spawn().unwrap().wait();
                }
            }
            out
        }
        AstNode::Assign { .. } => unreachable!(),
        AstNode::Binding { .. } => unreachable!(),
        AstNode::Function { .. } => unreachable!(),
        AstNode::QuoteString(qs) => {
            let parts: Vec<String> = qs
                .iter()
                .map(|ast| {
                    let val = eval_env(&ast, env);
                    let Some(Value::Str(s)) = val.convert(&Type::Str) else {
                        panic!()
                    };
                    s
                })
                .collect();
            Value::Str(parts.concat())
        }
        AstNode::StringLiteral(s) => Value::Str(String::from(s)),
        AstNode::Unit => todo!(),
        AstNode::IfThenElse {
            cond,
            t_block,
            f_block,
        } => {
            let out = eval_env(cond, env);
            let Value::Bool(b) = out else {
                todo!();
            };
            let out = if b {
                eval_env(t_block, env)
            } else {
                eval_env(f_block, env)
            };
            out
        }
        AstNode::RecordValue(r) => {
            let mut out = HashMap::new();
            for (key, ast) in r.iter() {
                let val = eval_env(ast, env);
                out.insert(key.clone(), val);
            }
            Value::Record(out)
        }
        AstNode::Call { name, args } => match (name.as_str(), args.as_slice()) {
            ("display", [e]) => {
                let out = eval_env(e, env);
                Value::Str(macro_display(out))
            }
            (name, args) => match env.fns.get(name) {
                Some(AstNode::Function {
                    name: _,
                    args: fn_args,
                    out: fn_out,
                    block,
                }) => {
                    let mut fn_env = Env {
                        vars: Vec::new(),
                        fns: env.fns.clone(),
                    };
                    for i in 0..args.len() {
                        let v = eval_env(&args[i], env);
                        fn_env
                            .vars
                            .push((fn_args[i].0.clone(), fn_args[i].1.clone(), v));
                    }

                    let v = eval_env(block, &fn_env);
                    v.convert(fn_out).unwrap()
                }
                Some(_) => {
                    todo!()
                }
                None => {
                    todo!()
                }
            },
        },
    }
}

fn macro_display(v: Value) -> String {
    match v {
        Value::Int(x) => x.to_string(),
        Value::Str(x) => x,
        Value::Bool(b) => (match b {
            true => "true",
            false => "false",
        })
        .into(),
        Value::List(xs) => {
            let ys: Vec<_> = xs.into_iter().map(macro_display).collect();
            ys.join(" ")
        }
        Value::Unit => "unit".into(),
        Value::Record(kv) => format!("{:?}", kv),
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    List(Vec<Value>),
    Unit,

    // Tuple(Vec<Value>),
    Record(HashMap<String, Value>),
    // Variant(String, Box<Value>),
    // Command { command: Rc<RefCell<Command>> },
    // Internal(TODO)
}

fn extract_command(val: &Value) -> Option<(String, Vec<String>)> {
    let Value::Record(r) = val else {
        return None;
    };
    let Value::Str(program) = r.get("program")? else {
        return None;
    };
    let Value::List(args) = r.get("args")? else {
        return None;
    };
    let args: Vec<String> = args
        .iter()
        .map(|v| match v {
            Value::Str(a) => a.clone(),
            _ => unreachable!(),
        })
        .collect();

    Some((program.clone(), args))
}

fn convert_record(val: Value, ty: &Type) -> Option<Value> {
    let Value::Record(r) = val else {
        return None;
    };

    // let t_process: Type = Type::Record(HashMap::from_iter([
    // (String::from("_process"), Type::Unit),
    // (String::from("_process"), Type::Unit),
    // (String::from("_process"), Type::Unit),
    // ]));

    match ty {
        Type::Str if r.contains_key("_command") => {
            // TODO: convert_record(convert_record(Value::Record(r), &t_process)?, &Type::Str)
            let Value::Str(program) = r.get("program")? else {
                todo!();
            };
            let Value::List(args) = r.get("args")? else {
                todo!();
            };
            let args: Vec<String> = args
                .iter()
                .map(|v| match v {
                    Value::Str(a) => a.clone(),
                    _ => unreachable!(),
                })
                .collect();

            let mut cmd = Command::new(program);
            cmd.args(args);
            let output = cmd.output().unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();
            Some(Value::Str(stdout.trim().to_string()))
        }
        // Type::Str if r.contains_key("_process") => todo!(),
        Type::Str => {
            todo!()
        }
        Type::Int => todo!(),
        Type::Bool => todo!(),
        Type::Unit => todo!(),
        Type::Float => todo!(),
        Type::List(_) => todo!(),
        Type::Map(_, _) => todo!(),
        Type::Tuple(_) => todo!(),
        Type::Variant(_) => todo!(),
        Type::Record(z) => {
            // for now, implement with markers only
            if r.contains_key("_command") && z.contains_key("_process") {
                todo!();
                // if let Value::Command { command } = &out {
                // let _ = command.borrow_mut().spawn().unwrap().wait();
                // }
                // return None;
            }

            let mut out = HashMap::new();
            for (key, t) in z.iter() {
                match r.get(key) {
                    Some(val) => match val.clone().convert(t) {
                        Some(c_val) => {
                            out.insert(key.clone(), c_val);
                        }
                        None => return None,
                    },
                    None => return None,
                }
            }
            Some(Value::Record(out))
        }
    }
}

// extremely temporary terrible prototype
impl Value {
    pub fn convert(self, ty: &Type) -> Option<Value> {
        match (&self, ty) {
            (&Value::Record(_), _) => convert_record(self, ty),
            (Value::Str(s), Type::Int) => Some(Value::Int(s.trim().parse().unwrap())),
            (Value::List(xs), Type::List(t)) => {
                let ys: Vec<_> = xs.iter().map(|x| x.clone().convert(t).unwrap()).collect();
                Some(Value::List(ys))
            }
            (Value::Str(s), Type::List(t)) => Some(Value::List(
                s.trim()
                    .split_whitespace()
                    .map(|x| Value::Str(String::from(x)).convert(t).unwrap())
                    .collect(),
            )),
            (Value::List(xs), Type::Str) => {
                let ys: Vec<_> = xs
                    .iter()
                    .map(|x| {
                        let Value::Str(v) = x.clone().convert(&Type::Str).unwrap() else {
                            todo!();
                        };
                        v
                    })
                    .collect();

                Some(Value::Str(ys.join(" ")))
            }
            (Value::Int(x), Type::Str) => Some(Value::Str(x.to_string())),
            // everything can be converted to itself
            (Value::Int(_), Type::Int) => Some(self),
            (Value::Str(_), Type::Str) => Some(self),
            (Value::Bool(_), Type::Bool) => Some(self),
            _ => None,
        }
    }
}
