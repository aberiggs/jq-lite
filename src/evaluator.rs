use crate::ast::Expr;
use serde_json::Value;

pub fn eval_expr(expr: &Expr, input: &Value) -> Value {
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
