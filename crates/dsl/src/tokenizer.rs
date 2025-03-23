use crate::{DSLError, DSLResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Identifier(&'a str),
    KeyWord(KeyWord),
    Number(i64),
    Char(char),
    String(&'a str),
}

impl Token<'_> {
    pub fn len(&self) -> usize {
        match self {
            Token::Identifier(ident) => ident.len(),
            Token::KeyWord(key_word) => key_word.len(),
            Token::Number(_) => todo!(),
            Token::Char(_) => 1,
            Token::String(string) => string.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() != 0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum KeyWord {
    AddAssign,
    Add,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Slice,
    SemiColon,
    Equal,
    If,
    Else,
    Comma,
    Let,
    Assign,
    Fn,
    While,
    Break,
    LessThan,
    NoMoreThan,
    MoreThan,
    NoLessThan,
    LOr,
    LAnd,
    Astar,
    MulAssign,
}

impl KeyWord {
    pub fn len(&self) -> usize {
        match self {
            KeyWord::AddAssign => 2,
            KeyWord::Add => 1,
            KeyWord::OpenSquareBracket => 1,
            KeyWord::CloseSquareBracket => 1,
            KeyWord::OpenParenthesis => 1,
            KeyWord::CloseParenthesis => 1,
            KeyWord::Slice => 2,
            KeyWord::SemiColon => 1,
            KeyWord::OpenBrace => 1,
            KeyWord::CloseBrace => 1,
            KeyWord::Equal => 2,
            KeyWord::If => 2,
            KeyWord::Else => 4,
            KeyWord::Comma => 1,
            KeyWord::Let => 3,
            KeyWord::Assign => 1,
            KeyWord::Fn => 2,
            KeyWord::While => 5,
            KeyWord::Break => 5,
            KeyWord::LessThan => 1,
            KeyWord::NoMoreThan => 2,
            KeyWord::MoreThan => 1,
            KeyWord::NoLessThan => 2,
            KeyWord::LOr => 2,
            KeyWord::LAnd => 2,
            KeyWord::Astar => 1,
            KeyWord::MulAssign => 2,
        }
    }

    pub fn is_empty(&self) -> bool {
        false
    }
}

pub fn tokenize<'a>(code: &'a str) -> DSLResult<Vec<Token<'a>>> {
    let mut counter: usize = 0;
    let mut token_seq: Vec<Token<'a>> = Vec::new();
    while counter < code.len() {
        match code.chars().nth(counter).unwrap() {
            whitespace if whitespace.is_ascii_whitespace() => {
                counter += 1;
                continue;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let begin = counter;
                while counter < code.len()
                    && matches!(code.chars().nth(counter).unwrap(), 'a'..='z' | 'A'..='Z' | '0'..='9' | '_')
                {
                    counter += 1;
                }
                match &code[begin..counter] {
                    "if" => token_seq.push(Token::KeyWord(KeyWord::If)),
                    "else" => token_seq.push(Token::KeyWord(KeyWord::Else)),
                    "let" => token_seq.push(Token::KeyWord(KeyWord::Let)),
                    "fn" => token_seq.push(Token::KeyWord(KeyWord::Fn)),
                    "while" => token_seq.push(Token::KeyWord(KeyWord::While)),
                    "break" => token_seq.push(Token::KeyWord(KeyWord::Break)),
                    _ => token_seq.push(Token::Identifier(&code[begin..counter])),
                }
                continue;
            }
            '0'..='9' => {
                let begin = counter;
                while counter < code.len()
                    && code.chars().nth(counter).unwrap().is_ascii_digit()
                {
                    counter += 1;
                }
                token_seq.push(Token::Number(code[begin..counter].parse().unwrap()));
                continue;
            }
            '=' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('=') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::Equal))
                    }
                    _ => token_seq.push(Token::KeyWord(KeyWord::Assign)),
                }
                continue;
            }
            '+' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('=') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::AddAssign))
                    }
                    _ => token_seq.push(Token::KeyWord(KeyWord::Add)),
                }
                continue;
            }
            '.' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('.') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::Slice))
                    }
                    _ => todo!(),
                }
                continue;
            }
            '[' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::OpenSquareBracket));
                continue;
            }
            ']' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::CloseSquareBracket));
                continue;
            }
            '(' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::OpenParenthesis));
                continue;
            }
            ')' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::CloseParenthesis));
                continue;
            }
            '{' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::OpenBrace));
                continue;
            }
            '}' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::CloseBrace));
                continue;
            }
            ';' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::SemiColon));
                continue;
            }
            '\"' => {
                counter += 1;
                let begin = counter;
                while !matches!(code.chars().nth(counter).unwrap(), '\"') {
                    counter += 1;
                }
                token_seq.push(Token::String(&code[begin..counter]));
                counter += 1;
                continue;
            }
            ',' => {
                counter += 1;
                token_seq.push(Token::KeyWord(KeyWord::Comma));
                continue;
            }
            '<' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('=') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::NoMoreThan))
                    }
                    _ => token_seq.push(Token::KeyWord(KeyWord::LessThan)),
                }
                continue;
            }
            '>' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('=') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::NoLessThan))
                    }
                    _ => token_seq.push(Token::KeyWord(KeyWord::MoreThan)),
                }
                continue;
            }
            '|' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('|') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::LOr))
                    }
                    _ => todo!(),
                }
                continue;
            }
            '&' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('&') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::LAnd))
                    }
                    _ => todo!(),
                }
                continue;
            }
            '*' => {
                counter += 1;
                match code.chars().nth(counter) {
                    Some('=') => {
                        counter += 1;
                        token_seq.push(Token::KeyWord(KeyWord::MulAssign))
                    }
                    _ => token_seq.push(Token::KeyWord(KeyWord::Astar)),
                }
                continue;
            }
            _ => {
                return Err(DSLError::Tokenize(format!(
                    "unexpected letter: {}",
                    code.chars().nth(counter).unwrap()
                )));
            }
        }
    }
    Ok(token_seq)
}

pub fn consume_token<'a>(
    expected_token: Token<'a>,
    token_seq: &Vec<Token<'a>>,
    counter: &mut usize,
) -> DSLResult<()> {
    if peek_token(token_seq, counter) == Some(expected_token) {
        *counter += 1;
        Ok(())
    } else {
        Err(DSLError::Parse(format!(
            "unexpected token: {:?},expected: {:?}",
            peek_token(token_seq, counter),
            expected_token
        )))
    }
}

pub fn peek_token<'a>(token_seq: &Vec<Token<'a>>, counter: &mut usize) -> Option<Token<'a>> {
    token_seq.get(*counter).copied()
}
