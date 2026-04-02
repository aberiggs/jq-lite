use crate::ast::Expr;
use crate::ast::PathSegment;

pub fn eval_expr(expr: &Expr, input: &serde_json::Value) -> Vec<serde_json::Value> {
    let mut val = input;

    let Expr::Path(path) = expr;
    let mut expr_iter = path.iter();

    while let Some(segment) = expr_iter.next() {
        if let serde_json::Value::Object(map) = val {
            match segment {
                PathSegment::Field(field) => {
                    if let Some(next) = map.get(field) {
                        val = next;
                    } else {
                        return vec![];
                    }
                }
                _ => return vec![],
            }
        } else if let serde_json::Value::Array(arr) = val {
            match segment {
                PathSegment::Index(index) => {
                    if let Some(next) = arr.get(*index) {
                        val = next;
                    } else {
                        return vec![];
                    }
                }
                PathSegment::Iter => {
                    // Assemble an new expr with the remaining segments we will traverse
                    let remaining: Vec<PathSegment> = expr_iter.cloned().collect();
                    let new_expr = Expr::Path(remaining);
                    let mut output = vec![];
                    for item in arr {
                        output.extend(eval_expr(&new_expr, item));
                    }
                    return output;
                }
                _ => return vec![],
            }
        } else {
            return vec![];
        }
    }

    vec![val.clone()]
}
