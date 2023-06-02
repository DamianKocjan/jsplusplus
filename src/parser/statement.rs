use super::expression::Expression;
use super::token::Token;

#[derive(Clone, Debug)]
pub enum Statement {
    Block {
        statements: Vec<Statement>,
    },
    Class {
        name: Token,
        superclass: Option<Expression>,
        // list of function statements
        methods: Vec<Statement>,
    },
    Expression {
        expression: Expression,
    },
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Print {
        expression: Expression,
    },
    Return {
        keyword: Token,
        value: Option<Expression>,
    },
    Let {
        name: Token,
        initializer: Option<Expression>,
    },
    Const {
        name: Token,
        initializer: Option<Expression>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
}

pub trait Visitor<T> {
    fn visit_block_statement(&mut self, statement: &Statement) -> T;
    fn visit_class_statement(&mut self, statement: &Statement) -> T;
    fn visit_expression_statement(&mut self, statement: &Statement) -> T;
    fn visit_function_statement(&mut self, statement: &Statement) -> T;
    fn visit_if_statement(&mut self, statement: &Statement) -> T;
    fn visit_print_statement(&mut self, statement: &Statement) -> T;
    fn visit_return_statement(&mut self, statement: &Statement) -> T;
    fn visit_let_statement(&mut self, statement: &Statement) -> T;
    fn visit_const_statement(&mut self, statement: &Statement) -> T;
    fn visit_while_statement(&mut self, statement: &Statement) -> T;
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Statement::Block { statements: _ } => visitor.visit_block_statement(self),
            Statement::Class {
                name: _,
                superclass: _,
                methods: _,
            } => visitor.visit_class_statement(self),
            Statement::Expression { expression: _ } => visitor.visit_expression_statement(self),
            Statement::Function {
                name: _,
                params: _,
                body: _,
            } => visitor.visit_function_statement(self),
            Statement::If {
                condition: _,
                then_branch: _,
                else_branch: _,
            } => visitor.visit_if_statement(self),
            Statement::Print { expression: _ } => visitor.visit_print_statement(self),
            Statement::Return {
                keyword: _,
                value: _,
            } => visitor.visit_return_statement(self),
            Statement::Let {
                name: _,
                initializer: _,
            } => visitor.visit_let_statement(self),
            Statement::Const {
                name: _,
                initializer: _,
            } => visitor.visit_const_statement(self),
            Statement::While {
                condition: _,
                body: _,
            } => visitor.visit_while_statement(self),
        }
    }
}
