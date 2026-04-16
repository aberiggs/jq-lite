mod ast;
mod evaluator;
mod lexer;
mod parser;

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

fn run(
    filter_txt: &str,
    input_json: &serde_json::Value,
) -> Result<Vec<serde_json::Value>, Box<dyn error::Error>> {
    let tokens = lexer::lex(filter_txt)?;
    dprintln!("Tokens: {:?}", tokens);
    let expr = parser::parse_tokens(tokens)?;
    dprintln!("Expr: {:?}", expr);
    let results = evaluator::eval_expr(&expr, input_json);

    Ok(results)
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

    let value: serde_json::Value = serde_json::from_str(&input_json_str)?;

    let results = run(filter_txt, &value)?;

    for result in &results {
        println!("{}", serde_json::to_string_pretty(result)?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".";
        let input_json = serde_json::json!({"field": 42});
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], serde_json::json!({"field": 42}));
        Ok(())
    }

    #[test]
    fn basic_field_access() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".field";
        let input_json = serde_json::json!({ "field": 42 });
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], serde_json::json!(42));
        Ok(())
    }

    #[test]
    fn nested_field_access() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".field.nested";
        let input_json = serde_json::json!({ "field": { "nested": 42 } });
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], serde_json::json!(42));
        Ok(())
    }

    #[test]
    fn incorrect_field_access() -> Result<(), Box<dyn error::Error>> {
        let input_json = serde_json::json!({ "a" : { "b": 4 } });

        assert!(run("a.b", &input_json).is_err());
        assert!(run(".a..b", &input_json).is_err());
        Ok(())
    }

    #[test]
    fn missing_field_access() -> Result<(), Box<dyn error::Error>> {
        let input_json = serde_json::json!({ "a": 42 });
        let results = run(".b", &input_json)?;
        assert_eq!(results.len(), 0);
        Ok(())
    }

    #[test]
    fn array_iteration() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".users[].name";
        let input_json = serde_json::json!({"users":[{"name":"Ada"},{"name":"Linus"}]});
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], serde_json::json!("Ada"));
        assert_eq!(results[1], serde_json::json!("Linus"));
        Ok(())
    }

    #[test]
    fn array_index_access() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".items[1]";
        let input_json = serde_json::json!({"items":[10,20,30]});
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], serde_json::json!(20));
        Ok(())
    }

    #[test]
    fn root_array_iteration() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".[]";
        let input_json = serde_json::json!([1, 2, 3]);
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 3);
        assert_eq!(results[0], serde_json::json!(1));
        assert_eq!(results[1], serde_json::json!(2));
        assert_eq!(results[2], serde_json::json!(3));
        Ok(())
    }

    #[test]
    fn out_of_bounds_array_index_access() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".items[5]";
        let input_json = serde_json::json!({"items":[10,20,30]});
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 0);
        Ok(())
    }

    #[test]
    fn array_iteration_on_non_array() -> Result<(), Box<dyn error::Error>> {
        let filter_txt = ".users[]";
        let input_json = serde_json::json!({"users": {"name": "Ada"}});
        let results = run(filter_txt, &input_json)?;
        assert_eq!(results.len(), 0);
        Ok(())
    }
}
