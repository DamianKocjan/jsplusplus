use crate::parser::token::TokenType;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    // Comments
    COMMENT,
    BlockComment,
}

pub static KEYWORDS: Lazy<Mutex<HashMap<&str, TokenType>>> = Lazy::new(|| {
    let mut m: HashMap<&str, TokenType> = HashMap::new();

    m.insert("and", TokenType::And);
    m.insert("const", TokenType::Const);
    m.insert("else", TokenType::Else);
    m.insert("false", TokenType::False);
    m.insert("for", TokenType::For);
    m.insert("function", TokenType::Function);
    m.insert("if", TokenType::If);
    m.insert("import", TokenType::Import);
    m.insert("let", TokenType::Let);
    m.insert("nil", TokenType::Nil);
    m.insert("or", TokenType::Or);
    m.insert("print", TokenType::Print);
    m.insert("return", TokenType::Return);
    m.insert("true", TokenType::True);
    m.insert("while", TokenType::While);
    Mutex::new(m)
});
