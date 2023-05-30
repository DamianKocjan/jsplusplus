use crate::parser::expression::{Expression, Visitor as ExpressionVisitor};
use crate::parser::statement::{Statement, Visitor as StatementVisitor};
use crate::parser::token::Token;

enum OneOf {
    Expression(Expression),
    Statement(Statement),
    Token(Token),
    Vec(Vec<Box<OneOf>>),
    String(String),
}

impl From<Expression> for OneOf {
    fn from(expression: Expression) -> Self {
        return OneOf::Expression(expression);
    }
}

impl From<Statement> for OneOf {
    fn from(statement: Statement) -> Self {
        return OneOf::Statement(statement);
    }
}

impl From<Token> for OneOf {
    fn from(token: Token) -> Self {
        return OneOf::Token(token);
    }
}

impl From<Vec<Box<OneOf>>> for OneOf {
    fn from(vec: Vec<Box<OneOf>>) -> Self {
        return OneOf::Vec(vec);
    }
}

impl From<Vec<Expression>> for OneOf {
    fn from(vec: Vec<Expression>) -> Self {
        let mut boxed_vec = Vec::new();

        for expr in vec {
            boxed_vec.push(Box::new(OneOf::Expression(expr)));
        }

        return OneOf::Vec(boxed_vec);
    }
}

impl From<String> for OneOf {
    fn from(string: String) -> Self {
        return OneOf::String(string);
    }
}

struct AstPrinter;

impl AstPrinter {
    fn print_expression(&mut self, expression: &Expression) -> String {
        return expression.accept(self);
    }

    fn print_statement(&mut self, statement: &Statement) -> String {
        return statement.accept(self);
    }

    fn parenthesize(&mut self, name: &str, exprs: &[Expression]) -> String {
        let mut builder = String::new();
        builder.push_str("(");
        builder.push_str(name);

        for expr in exprs {
            builder.push_str(" ");
            builder.push_str(&expr.accept(self));
        }

        builder.push_str(")");
        return builder;
    }

    fn parenthesize2(&mut self, name: &str, parts: &[Box<OneOf>]) -> String {
        let mut builder = String::new();
        builder.push_str(format!("({}", name).as_str());

        self.transform(&mut builder, parts);

        builder.push_str(")");
        return builder;
    }

    fn transform(&mut self, builder: &mut String, parts: &[Box<OneOf>]) {
        for part in parts {
            builder.push_str(" ");

            match part.as_ref() {
                OneOf::Expression(expression) => {
                    builder.push_str(&expression.accept(self));
                }
                OneOf::Statement(statement) => {
                    builder.push_str(&statement.accept(self));
                }
                OneOf::Token(token) => {
                    builder.push_str(&token.lexeme);
                }
                OneOf::Vec(vec) => {
                    self.transform(builder, vec);
                }
                OneOf::String(string) => {
                    builder.push_str(string);
                }
            }
        }
    }
}

impl ExpressionVisitor<String> for AstPrinter {
    fn visit_assign_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Assign { name, value } => self.parenthesize2(
                "assign",
                &[
                    Box::new(Into::<OneOf>::into(name.lexeme.clone())),
                    Box::new(Into::<OneOf>::into(value.as_ref().to_owned())),
                ],
            ),
            _ => {
                panic!("Expected Assign expression");
            }
        }
    }

    fn visit_binary_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Binary {
                left,
                operator,
                right,
            } => self.parenthesize(
                operator.lexeme.as_str(),
                &[left.as_ref().to_owned(), right.as_ref().to_owned()],
            ),
            _ => {
                panic!("Expected Binary expression");
            }
        }
    }

    fn visit_call_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Call {
                callee,
                paren: _,
                arguments,
            } => self.parenthesize2(
                "call",
                &[
                    Box::new(Into::<OneOf>::into(callee.as_ref().to_owned())),
                    Box::new(Into::<OneOf>::into(arguments.to_owned())),
                ],
            ),
            _ => {
                panic!("Expected Call expression");
            }
        }
    }

    fn visit_get_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Get { object, name } => self.parenthesize2(
                "get",
                &[
                    Box::new(Into::<OneOf>::into(object.as_ref().to_owned())),
                    Box::new(Into::<OneOf>::into(name.lexeme.clone())),
                ],
            ),
            _ => {
                panic!("Expected Get expression");
            }
        }
    }

    fn visit_grouping_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Grouping { expression } => {
                self.parenthesize("group", &[expression.as_ref().to_owned()])
            }
            _ => {
                panic!("Expected Grouping expression");
            }
        }
    }

    fn visit_literal_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Literal { value } => match value {
                Some(val) => val.to_string(),
                None => String::from("nil"),
            },
            _ => {
                panic!("Expected Literal expression");
            }
        }
    }

    fn visit_logical_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Logical {
                left,
                operator,
                right,
            } => self.parenthesize(
                operator.lexeme.as_str(),
                &[left.as_ref().to_owned(), right.as_ref().to_owned()],
            ),
            _ => {
                panic!("Expected Logical expression");
            }
        }
    }

    fn visit_set_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Set {
                object,
                name,
                value,
            } => self.parenthesize2(
                "=",
                &[
                    Box::new(Into::<OneOf>::into(object.as_ref().to_owned())),
                    Box::new(Into::<OneOf>::into(name.lexeme.clone())),
                    Box::new(Into::<OneOf>::into(value.as_ref().to_owned())),
                ],
            ),
            _ => {
                panic!("Expected Set expression");
            }
        }
    }

    fn visit_super_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Super { keyword: _, method } => {
                self.parenthesize2("super", &[Box::new(Into::<OneOf>::into(method.clone()))])
            }
            _ => {
                panic!("Expected Super expression");
            }
        }
    }

    fn visit_this_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::This { keyword: _ } => String::from("this"),
            _ => {
                panic!("Expected This expression");
            }
        }
    }

    fn visit_unary_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Unary { operator, right } => {
                self.parenthesize(operator.lexeme.as_str(), &[right.as_ref().to_owned()])
            }
            _ => {
                panic!("Expected Unary expression");
            }
        }
    }

    fn visit_variable_expression(&mut self, expr: &Expression) -> String {
        match expr {
            Expression::Variable { name } => name.lexeme.clone(),
            _ => {
                panic!("Expected Variable expression");
            }
        }
    }
}

impl StatementVisitor<String> for AstPrinter {
    fn visit_block_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Block { statements } => {
                let mut builder = String::new();
                builder.push_str("(block ");

                for statement in statements {
                    builder.push_str(&self.print_statement(statement));
                }

                builder.push_str(")");
                return builder;
            }
            _ => {
                panic!("Expected block statement");
            }
        }
    }

    fn visit_class_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Class {
                name,
                superclass,
                methods,
            } => {
                let mut builder = String::new();
                builder.push_str(format!("(class {} ", name.lexeme).as_str());

                if superclass.is_some() {
                    builder.push_str(&format!(
                        " < {}",
                        self.print_expression(superclass.as_ref().unwrap())
                    ));
                }

                for method in methods {
                    match method {
                        Statement::Function {
                            name: _,
                            params: _,
                            body: _,
                        } => {
                            builder.push_str(format!(" {}", self.print_statement(method)).as_str());
                        }
                        _ => {
                            panic!("Expected function statement");
                        }
                    }
                }

                return builder;
            }
            _ => {
                panic!("Expected class statement");
            }
        }
    }

    fn visit_expression_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Expression { expression } => {
                return self.parenthesize(";", &[expression.clone()]);
            }
            _ => {
                panic!("Expected expression statement");
            }
        }
    }

    fn visit_function_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Function { name, params, body } => {
                let mut builder = String::new();
                builder.push_str(format!("(fun {}(", name.lexeme).as_str());

                for param in params {
                    if param != &params[0] {
                        builder.push_str(" ");
                    }
                    builder.push_str(param.lexeme.as_str());
                }

                builder.push_str(") ");

                for body in body {
                    builder.push_str(&body.accept(self));
                }

                builder.push_str(")");
                return builder;
            }
            _ => {
                panic!("Expected function statement");
            }
        }
    }

    fn visit_if_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => match else_branch {
                Some(else_branch) => {
                    return self.parenthesize2(
                        "if",
                        &[
                            // it's so bad XD
                            Box::new(Into::<OneOf>::into(condition.to_owned())),
                            Box::new(Into::<OneOf>::into(then_branch.as_ref().to_owned())),
                            Box::new(Into::<OneOf>::into(else_branch.as_ref().to_owned())),
                        ],
                    );
                }
                None => {
                    return self.parenthesize2(
                        "if",
                        &[
                            Box::new(Into::<OneOf>::into(condition.to_owned())),
                            Box::new(Into::<OneOf>::into(then_branch.as_ref().to_owned())),
                        ],
                    );
                }
            },
            _ => {
                panic!("Expected if statement");
            }
        }
    }

    fn visit_print_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Print { expression } => {
                return self.parenthesize("print", &[expression.clone()]);
            }
            _ => {
                panic!("Expected print statement");
            }
        }
    }

    fn visit_return_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Return { keyword: _, value } => {
                if value.is_some() {
                    return self.parenthesize("return", &[value.clone().unwrap()]);
                }
                return self.parenthesize("return", &[]);
            }
            _ => {
                panic!("Expected return statement");
            }
        }
    }

    fn visit_var_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::Var { name, initializer } => {
                if initializer.is_some() {
                    return self.parenthesize2(
                        "var",
                        &[
                            Box::new(Into::<OneOf>::into(name.to_owned())),
                            Box::new(Into::<OneOf>::into(String::from("="))),
                            Box::new(Into::<OneOf>::into(
                                initializer.as_ref().unwrap().to_owned(),
                            )),
                        ],
                    );
                }
                return format!("(var {})", name.lexeme);
            }
            _ => {
                panic!("Expected var statement");
            }
        }
    }

    fn visit_while_statement(&mut self, stmt: &Statement) -> String {
        match stmt {
            Statement::While { condition, body } => {
                return self.parenthesize2(
                    "while",
                    &[
                        Box::new(Into::<OneOf>::into(condition.to_owned())),
                        Box::new(Into::<OneOf>::into(body.as_ref().to_owned())),
                    ],
                );
            }
            _ => {
                panic!("Expected while statement");
            }
        }
    }
}
