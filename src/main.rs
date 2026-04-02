mod ast;
mod evaluator;
mod lexer;
mod parser;

use serde_json::Value;

use std::env;
use std::error;
use std::io;
use std::io::Read;

macro_rules! dprintln {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        println!($($arg)*);
    };
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: jq-lite '<filter>'");
        return Err("Invalid number of arguments!".into());
    }

    let mut input_json_str = String::new();
    io::stdin().read_to_string(&mut input_json_str)?;

    let filter_txt = &args[1].trim();

    let value: Value = serde_json::from_str(&input_json_str)?;

    let tokens = lexer::lex(filter_txt)?;
    dprintln!("Tokens: {:?}", tokens);
    let expr = parser::parse_tokens(tokens)?;
    dprintln!("Expr: {:?}", expr);
    let results = evaluator::eval_expr(&expr, &value);

    for result in &results {
        println!("{}", serde_json::to_string_pretty(result)?);
    }

    Ok(())
}
