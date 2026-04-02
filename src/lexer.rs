#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Dot,
    Identifier(String),
    LBracket,
    RBracket,
    Number(String),
}

#[derive(Debug)]
pub struct LexError(String);

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lex error: {}", self.0)
    }
}

impl std::error::Error for LexError {}

pub fn lex(filter_txt: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut iter = filter_txt.char_indices().peekable();

    while let Some((pos, c)) = iter.next() {
        match c {
            '.' => tokens.push(Token::Dot),
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut ident = String::from(c);
                while let Some(&(_, next)) = iter.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' || next == '-' {
                        ident.push(next);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(ident));
            }
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            c if c.is_ascii_digit() => {
                let mut num = String::from(c);
                let mut found_decimal = false;
                while let Some(&(_, next)) = iter.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        if next == '.' {
                            if found_decimal {
                                return Err(LexError(format!(
                                    "Found multiple decimal points in number at position {}",
                                    pos
                                )));
                            }
                            found_decimal = true;
                        }
                        num.push(next);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num));
            }
            c if c.is_whitespace() => {
                // Eat whitespace
            }
            _ => {
                return Err(LexError(format!(
                    "Unexpected character: {:?} at pos {}",
                    c, pos
                )));
            }
        }
    }

    Ok(tokens)
}
