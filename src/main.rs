pub mod ast;
pub mod interpreter;
pub mod parse;
pub mod types;

use parse::KleyParser;

fn main() {
    let code = include_str!("demo.kley");
    // println!("{}", code);
    let ast = match KleyParser::build_ast(code) {
        Ok(ast) => ast,
        Err(err) => {
            println!("{err}");
            return;
        }
    };

    // println!("{:?}", ast);

    let (v, env) = interpreter::eval(&ast);
    // println!("{:?}", v);
    // println!("{:?}", env);
}
