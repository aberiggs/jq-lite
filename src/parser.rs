use crate::ast::Expr;
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

    // TODO: Update this so that the parser looks forward rather than backward.
    let mut prev_token = &tokens[0];
    // NOTE: Revisit. `Expr` won't only be a `Path` in the future.
    let mut expr = Expr::Path(vec![]);
    for token in &tokens[1..] {
        match token {
            Token::Dot => {
                if prev_token == &Token::Dot {
                    return Err(ParseError(String::from("Repeated '.' in path expression!")));
                }
            }
            Token::Identifier(ident) => {
                let Expr::Path(path) = &mut expr;
                path.push(ident.clone());
            }
        }

        prev_token = token;
    }

    let Expr::Path(path) = &expr;
    if prev_token == &Token::Dot && !path.is_empty() {
        return Err(ParseError(String::from(
            "Path expression must end with an identifier!",
        )));
    }

    Ok(expr)
}
