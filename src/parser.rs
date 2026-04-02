use crate::ast::Expr;
use crate::ast::PathSegment;
use crate::lexer::Token;

#[derive(Debug)]
pub struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parsing error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    if tokens.is_empty() {
        return Err(ParseError(String::from("No tokens to parse!")));
    } else if tokens[0] != Token::Dot {
        return Err(ParseError(String::from(
            "Filter expression must start with a '.'!",
        )));
    }

    // NOTE: Revisit. `Expr` won't only be a `Path` in the future.
    let mut expr = Expr::Path(vec![]);

    let mut tok_iter = tokens.iter().peekable();
    while let Some(token) = tok_iter.next() {
        match token {
            Token::Dot => {
                let Some(&next_token) = tok_iter.peek() else {
                    // An expression should never end with a `Dot` unless it's the only token in the expression
                    if tokens.len() > 1 {
                        return Err(ParseError(String::from(
                            "Path expression cannot end with a '.'!",
                        )));
                    }

                    break;
                };

                // Check for the next expected tokens
                if !matches!(next_token, Token::Identifier(_) | Token::LBracket) {
                    return Err(ParseError(format!(
                        "Invalid token '{:?}' after '.'!",
                        next_token
                    )));
                }
            }
            Token::Identifier(ident) => {
                let Expr::Path(path) = &mut expr;
                path.push(PathSegment::Field(ident.clone()));

                if let Some(&next_token) = tok_iter.peek() {
                    // If not the end, check that the next token is either a '.' or '['
                    if !matches!(next_token, Token::Dot | Token::LBracket) {
                        return Err(ParseError(format!(
                            "Invalid token '{:?}' after field access '{}'!",
                            next_token, ident
                        )));
                    }
                }
            }
            Token::LBracket => {
                // Two valid outcomes from here.
                // A. Index access (e.g., .[0])
                // B. Iteration (e.g., .[])

                let Some(&next_token) = tok_iter.peek() else {
                    return Err(ParseError(String::from("Expected a token to follow '['!")));
                };

                match next_token {
                    Token::Number(idx_str) => {
                        // First, check if idx_str is even a valid index
                        let idx: usize = idx_str.parse().map_err(|_| {
                            ParseError(String::from("Non-integer index when accessing an array"))
                        })?;

                        // Eat the number token
                        tok_iter.next();

                        // Now check to make sure we have a closing bracket
                        let Some(&next_next_token) = tok_iter.peek() else {
                            return Err(ParseError(format!("Missing ']' after '[{}'!", idx_str,)));
                        };

                        if !matches!(next_next_token, Token::RBracket) {
                            return Err(ParseError(format!(
                                "Expected ']' after '[{}', but got '{:?}'!",
                                idx_str, next_next_token
                            )));
                        }

                        // Now eat the closing bracket
                        tok_iter.next();

                        let Expr::Path(path) = &mut expr;
                        path.push(PathSegment::Index(idx));
                    }
                    Token::RBracket => {
                        // Eat the closing bracket
                        tok_iter.next();

                        let Expr::Path(path) = &mut expr;
                        path.push(PathSegment::Iter);
                    }
                    _ => {
                        return Err(ParseError(format!(
                            "Invalid token '{:?}' after '['!",
                            next_token
                        )));
                    }
                }

                // Now that we've parsed the whole index expression, check that the next token is valid
                if let Some(&next_token) = tok_iter.peek() {
                    // If not the end, check that the next token is either a '.' or '['
                    if !matches!(next_token, Token::Dot | Token::LBracket) {
                        return Err(ParseError(format!("Unexpected token {:?}!", next_token)));
                    }
                }
            }
            _ => {
                return Err(ParseError(format!("Unexpected token {:?}!", token)));
            }
        }
    }

    Ok(expr)
}
