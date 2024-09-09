pub mod ast;
pub mod interpreter;
pub mod parse;
pub mod types;

use std::error::Error;

use clap::Parser;
use parse::{KleyParser, Rule};
use pest::Parser as _;

/// Kley language implementation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Kley script to process
    file: String,

    #[arg(long)]
    debug_peg: bool,

    #[arg(long)]
    debug_ast: bool,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run_interpreter(&args) {
        eprintln!("{e}");
        return;
    };
}

fn run_interpreter(args: &Args) -> Result<(), Box<dyn Error>> {
    let code = std::fs::read_to_string(args.file.clone())?;
    let mut pairs = KleyParser::parse(Rule::program, &code)?;

    if args.debug_peg {
        parse::display_pairs(&mut pairs, 0);
        return Ok(());
    }
    let ast = parse::build_ast(pairs)?;

    if args.debug_ast {
        println!("{:#?}", ast);
    }

    let _ = interpreter::eval(&ast);

    Ok(())
}

// TODO: JIT compilation

// TODO: bash transpiler
