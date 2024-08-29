use std::{cell::RefCell, process::Command, rc::Rc};

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Str,
    Bool,
    List(Box<Type>),
    Unit,
    // TODO: Tuple(Vec<VarType>),
    // TODO: Map(Box<VarType>, Box<VarType>),
    // TODO: Struct, Enum
    // Command {
    // text: String,
    // stdin: CommandIO,
    // stdout: CommandIO,
    // },
    // Child {},
}

impl Type {
    pub fn parse(tstr: &str) -> Self {
        match tstr {
            "int" => Self::Int,
            "str" => Self::Str,
            "bool" => Self::Bool,
            // extremely terrible temporary
            "list<int>" => Self::List(Box::new(Self::Int)),
            "list<str>" => Self::List(Box::new(Self::Str)),
            "list<bool>" => Self::List(Box::new(Self::Bool)),
            _ => todo!("unimplemented: {}", tstr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Str(String),
    Bool(bool),
    List(Vec<Value>),
    Unit,

    Command { command: Rc<RefCell<Command>> },
}

// extremely temporary terrible prototype
impl Value {
    pub fn convert(self, ty: &Type) -> Option<Value> {
        match (&self, ty) {
            (Value::Command { command }, Type::Str) => {
                let output = command.borrow_mut().output().unwrap();
                let stdout = String::from_utf8(output.stdout).unwrap();
                Some(Value::Str(stdout))
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
            // everything can be converted to itself
            (Value::Int(_), Type::Int) => Some(self),
            (Value::Str(_), Type::Str) => Some(self),
            (Value::Bool(_), Type::Bool) => Some(self),
            _ => None,
        }
    }
}
