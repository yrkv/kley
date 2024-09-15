use std::{cell::RefCell, collections::HashMap, process::Command, rc::Rc};

use pest::iterators::{Pair, Pairs};

use crate::parse::Rule;

#[derive(Debug, Clone)]
pub enum Type {
    Str,
    Int,
    Bool,
    Unit,
    Float,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Variant(HashMap<String, Type>),
    Record(HashMap<String, Type>),
}

fn next_string(pairs: &mut Pairs<Rule>) -> String {
    pairs.next().unwrap().as_str().trim().to_owned()
}

fn next_type(pairs: &mut Pairs<Rule>) -> Type {
    Type::parse(pairs.next().unwrap())
}

impl Type {
    pub fn parse(pair: Pair<Rule>) -> Self {
        let rule = pair.as_rule();
        let mut inner = pair.into_inner();
        match rule {
            Rule::t_str => Self::Str,
            Rule::t_int => Self::Int,
            Rule::t_bool => Self::Bool,
            Rule::t_unit => Self::Unit,
            Rule::t_float => Self::Float,
            Rule::t_list => Self::List(Box::new(next_type(&mut inner))),
            Rule::t_map => Self::Map(
                Box::new(next_type(&mut inner)),
                Box::new(next_type(&mut inner)),
            ),
            Rule::t_tuple => Self::Tuple(inner.map(Self::parse).collect()),
            Rule::t_variant | Rule::t_record => {
                let mut out = HashMap::new();
                while inner.peek().is_some() {
                    let field_name = next_string(&mut inner);
                    let field_type = next_type(&mut inner);
                    if out.contains_key(&field_name) {
                        todo!("what to do with duplicate fields?");
                    }
                    out.insert(field_name, field_type);
                }
                match rule {
                    Rule::t_variant => Self::Variant(out),
                    Rule::t_record => Self::Record(out),
                    _ => unreachable!(),
                }
            }
            Rule::t_ident => todo!(),
            // Rule::r#type => todo!(),
            _ => unimplemented!(),
        }
    }
    // pub fn parse(tstr: &str) -> Self {
    //     match tstr {
    //         "int" => Self::Int,
    //         "str" => Self::Str,
    //         "bool" => Self::Bool,
    //         // extremely terrible temporary
    //         "list<int>" => Self::List(Box::new(Self::Int)),
    //         "list<str>" => Self::List(Box::new(Self::Str)),
    //         "list<bool>" => Self::List(Box::new(Self::Bool)),
    //         "t_foo" => Self::Record(HashMap::from([
    //             (String::from("a"), Type::Str),
    //             (String::from("b"), Type::Str),
    //             (String::from("c"), Type::Str),
    //         ])),
    //         "t_bar" => Self::Record(HashMap::from([
    //             (String::from("a"), Type::Int),
    //             (String::from("b"), Type::Int),
    //         ])),
    //         _ => todo!("unimplemented: {}", tstr),
    //     }
    // }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    List(Vec<Value>),
    Unit,

    Command { command: Rc<RefCell<Command>> },

    // Tuple(Vec<Value>),
    Record(HashMap<String, Value>),
}

// extremely temporary terrible prototype
impl Value {
    pub fn convert(self, ty: &Type) -> Option<Value> {
        match (&self, ty) {
            (Value::Command { command }, Type::Str) => {
                let output = command.borrow_mut().output().unwrap();
                let stdout = String::from_utf8(output.stdout).unwrap();
                Some(Value::Str(stdout.trim().to_string()))
            }
            (Value::Command { command: _ }, Type::Int) => {
                self.convert(&Type::Str).unwrap().convert(ty)
            }
            (Value::Command { command: _ }, Type::List(_)) => {
                self.convert(&Type::Str).unwrap().convert(ty)
            }
            // actual conversions
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
            //
            (Value::Record(r1), Type::Record(r2)) => {
                let mut out = HashMap::new();
                for (key, t) in r2.iter() {
                    match r1.get(key) {
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
            (Value::Record(r1), Type::Str) => Some(Value::Str(format!("{:?}", r1))),
            _ => None,
        }
    }
}
