use std::collections::HashMap;

use super::token::Token;

#[derive(Clone, Debug)]
pub enum Literal {
    Nil,
    Bool(bool),
    Number(f64),
    String(String),
    Map(HashMap<String, String>),
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Literal::Nil => "nil".to_string(),
            Literal::Bool(value) => value.to_string(),
            Literal::Number(value) => value.to_string(),
            Literal::String(value) => value.to_string(),
            Literal::Map(value) => value
                .into_iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect::<Vec<String>>()
                .join(", "),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Assign {
        name: Token,
        value: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        paren: Token,
        arguments: Vec<Expression>,
    },
    Get {
        object: Box<Expression>,
        name: Token,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Literal {
        value: Option<Literal>,
    },
    Logical {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Set {
        object: Box<Expression>,
        name: Token,
        value: Box<Expression>,
    },
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Variable {
        name: Token,
    },
}

pub trait Visitor<T> {
    fn visit_assign_expression(&mut self, expr: &Expression) -> T;
    fn visit_binary_expression(&mut self, expr: &Expression) -> T;
    fn visit_call_expression(&mut self, expr: &Expression) -> T;
    fn visit_get_expression(&mut self, expr: &Expression) -> T;
    fn visit_grouping_expression(&mut self, expr: &Expression) -> T;
    fn visit_literal_expression(&mut self, expr: &Expression) -> T;
    fn visit_logical_expression(&mut self, expr: &Expression) -> T;
    fn visit_set_expression(&mut self, expr: &Expression) -> T;
    fn visit_unary_expression(&mut self, expr: &Expression) -> T;
    fn visit_variable_expression(&mut self, expr: &Expression) -> T;
}

impl Expression {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expression::Assign { name: _, value: _ } => visitor.visit_assign_expression(self),
            Expression::Binary {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_binary_expression(self),
            Expression::Call {
                callee: _,
                paren: _,
                arguments: _,
            } => visitor.visit_call_expression(self),
            Expression::Get { object: _, name: _ } => visitor.visit_get_expression(self),
            Expression::Grouping { expression: _ } => visitor.visit_grouping_expression(self),
            Expression::Literal { value: _ } => visitor.visit_literal_expression(self),
            Expression::Logical {
                left: _,
                operator: _,
                right: _,
            } => visitor.visit_logical_expression(self),
            Expression::Set {
                object: _,
                name: _,
                value: _,
            } => visitor.visit_set_expression(self),
            Expression::Unary {
                operator: _,
                right: _,
            } => visitor.visit_unary_expression(self),
            Expression::Variable { name: _ } => visitor.visit_variable_expression(self),
        }
    }
}
