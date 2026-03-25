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

    let query = &args[1];

    let value: Value = serde_json::from_str(&input_json_str)?;

    if query != "." {
        return Err(format!("Unsupported filter: {}", query).into());
    }

    println!("{}", serde_json::to_string_pretty(&value)?);

    Ok(())
}
