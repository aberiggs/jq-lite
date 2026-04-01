mod ast;
mod evaluator;
mod parser;

use evaluator::eval_expr;
use parser::parse_filter;

use serde_json::Value;

use std::env;
use std::error;
use std::io;
use std::io::Read;

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

    let expr = parse_filter(filter_txt)?;
    let result = eval_expr(&expr, &value);

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
