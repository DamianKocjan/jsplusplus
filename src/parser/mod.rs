mod environment;
pub mod expression;
mod io;
mod keyword;
pub mod scanner;
pub mod statement;
pub mod token;

pub struct Parser {
    tokens: Vec<token::Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<token::Token>) -> Parser {
        Parser { tokens, current: 0 }
    }
}
