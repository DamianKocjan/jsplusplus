use super::token::Token;

#[derive(Clone)]
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
        value: Option<Token>,
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
    Super {
        keyword: Token,
        method: Token,
    },
    This {
        keyword: Token,
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
    fn visit_super_expression(&mut self, expr: &Expression) -> T;
    fn visit_this_expression(&mut self, expr: &Expression) -> T;
    fn visit_unary_expression(&mut self, expr: &Expression) -> T;
    fn visit_variable_expression(&mut self, expr: &Expression) -> T;
}

impl Expression {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Expression::Assign { name, value } => visitor.visit_assign_expression(self),
            Expression::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expression(self),
            Expression::Call {
                callee,
                paren,
                arguments,
            } => visitor.visit_call_expression(self),
            Expression::Get { object, name } => visitor.visit_get_expression(self),
            Expression::Grouping { expression } => visitor.visit_grouping_expression(self),
            Expression::Literal { value } => visitor.visit_literal_expression(self),
            Expression::Logical {
                left,
                operator,
                right,
            } => visitor.visit_logical_expression(self),
            Expression::Set {
                object,
                name,
                value,
            } => visitor.visit_set_expression(self),
            Expression::Super { keyword, method } => visitor.visit_super_expression(self),
            Expression::This { keyword } => visitor.visit_this_expression(self),
            Expression::Unary { operator, right } => visitor.visit_unary_expression(self),
            Expression::Variable { name } => visitor.visit_variable_expression(self),
        }
    }
}
