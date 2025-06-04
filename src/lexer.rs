#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Gene,
    Inputs,
    Outputs,
    Logic,
    Express,
    Print,
    Identifier(String),
    StringLiteral(String),
    Equals,
    Plus,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    Comma,
    Arrow,
    Eof,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '{' => { tokens.push(Token::LBrace); chars.next(); }
            '}' => { tokens.push(Token::RBrace); chars.next(); }
            '[' => { tokens.push(Token::LBracket); chars.next(); }
            ']' => { tokens.push(Token::RBracket); chars.next(); }
            '(' => { tokens.push(Token::LParen); chars.next(); }
            ')' => { tokens.push(Token::RParen); chars.next(); }
            ':' => { tokens.push(Token::Colon); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::Equals);
                }
            }
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '"' => {
                chars.next(); // skip "
                let mut s = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' {
                        break;
                    }
                    s.push(c);
                    chars.next();
                }
                chars.next(); // skip ending "
                tokens.push(Token::StringLiteral(s));
            }
            c if c.is_whitespace() => { chars.next(); }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() || c2 == '_' {
                        ident.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let keyword = match ident.as_str() {
                    "gene" => Token::Gene,
                    "inputs" => Token::Inputs,
                    "outputs" => Token::Outputs,
                    "logic" => Token::Logic,
                    "express" => Token::Express,
                    "print" => Token::Print,
                    _ => Token::Identifier(ident),
                };
                tokens.push(keyword);
            }
            _ => {
                println!("Unexpected character: {}", ch);
                chars.next();
            }
        }
    }

    tokens.push(Token::Eof);
    tokens
}
