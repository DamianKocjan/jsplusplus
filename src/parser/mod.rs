use crate::JSPlusPlus;

use self::{
    expression::{Expression, Literal},
    statement::Statement,
    token::{Token, TokenType},
};
use anyhow::{bail, Result};

mod environment;
pub mod expression;
mod keyword;
mod resolver;
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

    pub fn parse_expression(&mut self) -> Result<Expression> {
        self.expression()
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            let declaration = self.declaration();
            match declaration {
                Some(declaration) => statements.push(declaration),
                None => {}
            }
        }
        statements
    }

    fn expression(&mut self) -> Result<Expression> {
        self.assignment()
    }

    fn declaration(&mut self) -> Option<Statement> {
        let mut get_stmt = || -> Result<Statement> {
            if self._match(&[TokenType::Function]) {
                return self.function("function");
            }
            if self._match(&[TokenType::Let]) {
                return self.let_declaration();
            }
            if self._match(&[TokenType::Const]) {
                return self.const_declaration();
            }
            return self.statement();
        };

        let result = get_stmt();
        if result.is_err() {
            self.synchronize();
            return None;
        }
        Some(result.unwrap())
    }

    fn statement(&mut self) -> Result<Statement> {
        if self._match(&[TokenType::For]) {
            return Ok(self.for_statement().unwrap());
        }
        if self._match(&[TokenType::If]) {
            return Ok(self.if_statement().unwrap());
        }
        if self._match(&[TokenType::Print]) {
            return Ok(self.print_statement().unwrap());
        }
        if self._match(&[TokenType::Return]) {
            return Ok(self.return_statement().unwrap());
        }
        if self._match(&[TokenType::While]) {
            return Ok(self.while_statement().unwrap());
        }
        if self._match(&[TokenType::LeftBrace]) {
            return Ok(Statement::Block {
                statements: self.block().unwrap(),
            });
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer: Option<Statement>;
        if self._match(&[TokenType::Semicolon]) {
            initializer = None;
        } else if self._match(&[TokenType::Let]) {
            initializer = Some(self.let_declaration()?);
        } else if self._match(&[TokenType::Const]) {
            initializer = Some(self.const_declaration()?);
        } else {
            initializer = Some(self.expression_statement()?);
        }

        let mut condition: Option<Expression> = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment: Option<Expression> = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Statement::Block {
                statements: vec![
                    body,
                    Statement::Expression {
                        expression: increment,
                    },
                ],
            };
        }

        if condition.is_none() {
            condition = Some(Expression::Literal {
                value: Some(Literal::Bool(true)),
            });
        }
        body = Statement::While {
            condition: condition.unwrap(),
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Statement::Block {
                statements: vec![initializer, body],
            };
        }
        Ok(body)
    }

    fn if_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let mut else_branch: Option<Box<Statement>> = None;
        if self._match(&[TokenType::Else]) {
            else_branch = Some(Box::new(self.statement()?));
        }

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn print_statement(&mut self) -> Result<Statement> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Print { expression: value })
    }

    fn return_statement(&mut self) -> Result<Statement> {
        let keyword = self.previous();
        let mut value: Option<Expression> = None;
        if !self.check(TokenType::Semicolon) {
            value = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Statement::Return { keyword, value })
    }

    fn let_declaration(&mut self) -> Result<Statement> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer: Option<Expression> = None;
        if self._match(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Statement::Let { name, initializer })
    }

    fn const_declaration(&mut self) -> Result<Statement> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer: Option<Expression> = None;
        if self._match(&[TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Statement::Const { name, initializer })
    }

    fn while_statement(&mut self) -> Result<Statement> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = Box::new(self.statement()?);

        Ok(Statement::While { condition, body })
    }

    fn expression_statement(&mut self) -> Result<Statement> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement::Expression { expression: expr })
    }

    fn function(&mut self, kind: &str) -> Result<Statement> {
        let name = self.consume(TokenType::Identifier, &format!("Expect {} name.", kind))?;
        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 parameters.");
                }
                parameters.push(self.consume(TokenType::Identifier, "Expect parameter name.")?);
                if !self._match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;
        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        Ok(Statement::Function {
            name,
            params: parameters,
            body,
        })
    }

    fn block(&mut self) -> Result<Vec<Statement>> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let statement = self.declaration();

            if let Some(statement) = statement {
                statements.push(statement);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn assignment(&mut self) -> Result<Expression> {
        let expr = self.or()?;

        if self._match(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment()?;

            if let Expression::Variable { name } = expr {
                return Ok(Expression::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            self.error(equals, "Invalid assignment target.");
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expression> {
        let mut expr = self.and()?;

        while self._match(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expression> {
        let mut expr = self.equality()?;

        while self._match(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expression::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression> {
        let mut expr = self.comparison()?;

        while self._match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression> {
        let mut expr = self.term()?;

        while self._match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression> {
        let mut expr = self.factor()?;

        while self._match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expression> {
        let mut expr = self.unary()?;

        while self._match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expression::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression> {
        if self._match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary()?;
            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.call()
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Can't have more than 255 arguments.");
                }
                arguments.push(self.expression()?);
                if !self._match(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;

        Ok(Expression::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        })
    }

    fn call(&mut self) -> Result<Expression> {
        let mut expr = self.primary()?;

        loop {
            if self._match(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn primary(&mut self) -> Result<Expression> {
        if self._match(&[TokenType::False]) {
            return Ok(Expression::Literal {
                value: Some(expression::Literal::Bool(false)),
            });
        }
        if self._match(&[TokenType::True]) {
            return Ok(Expression::Literal {
                value: Some(expression::Literal::Bool(true)),
            });
        }
        if self._match(&[TokenType::Nil]) {
            return Ok(Expression::Literal { value: None });
        }

        if self._match(&[TokenType::Number, TokenType::String]) {
            return Ok(Expression::Literal {
                value: Some(Literal::Map(self.previous().literal.unwrap())),
            });
        }

        if self._match(&[TokenType::Identifier]) {
            return Ok(Expression::Variable {
                name: self.previous(),
            });
        }

        if self._match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expression::Grouping {
                expression: Box::new(expr),
            });
        }

        bail!(self.error(self.peek(), "Expect expression."),)
    }

    fn _match(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type.clone()) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        bail!(self.error(self.peek(), message))
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn error(&self, token: Token, message: &str) -> String {
        JSPlusPlus::error_token(&token, message);
        format!("{}: {}", token.line, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Function
                | TokenType::Let
                | TokenType::Const
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}
