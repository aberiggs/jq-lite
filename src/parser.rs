use crate::ast::Expr;

#[derive(Debug)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_filter(filter_txt: &str) -> Result<Expr, ParseError> {
    if filter_txt.starts_with('.') {
        if filter_txt == "." {
            // Identity filter: return empty path
            return Ok(Expr::Path(vec![]));
        }

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
