use serde_json::Value;
use std::env;
use std::error;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}

enum Expr {
    Path(Vec<String>),
}

fn parse_filter(filter_txt: &str) -> Result<Expr, ParseError> {
    if filter_txt.starts_with('.') {
        // Split filter txt into segments, excluding the leading '.'
        let segments: Vec<String> = filter_txt[1..].split('.').map(|s| s.to_string()).collect();
        // Check if there are any empty segments (which shouldn't be allowed)
        if segments.iter().any(|s| s.is_empty()) {
            return Err(ParseError(String::from(
                "Invalid filter string: empty segment in path!",
            )));
        }

        return Ok(Expr::Path(segments));
    }

    Err(ParseError(String::from("Unrecognized filter string!")))
}

fn eval_expr(expr: &Expr, input: &Value) -> Value {
    match expr {
        Expr::Path(segments) => {
            let mut cur = input;
            for segment in segments {
                match cur {
                    Value::Object(map) => {
                        if let Some(next) = map.get(segment) {
                            cur = next;
                        } else {
                            return Value::Null;
                        }
                    }
                    _ => return Value::Null,
                }
            }
            cur.clone()
        }
    }
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

    let expr = parse_filter(filter_txt)?;
    let result = eval_expr(&expr, &value);

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
