#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Dot,
    Identifier(String),
}

pub fn lex(filter_txt: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = filter_txt.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '.' => tokens.push(Token::Dot),
            _ if c.is_alphanumeric() || c == '_' || c == '-' => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' || next == '-' {
                        ident.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(ident));
            }
            _ => {}
        }
    }

    tokens
}
