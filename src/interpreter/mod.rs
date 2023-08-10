use std::any::Any;

use anyhow::{bail, Ok, Result};

use crate::{
    parser::{
        expression::{Expression, Visitor as ExpressionVisitor},
        token::{Token, TokenType},
    },
    JSPlusPlus, RuntimeError,
};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn interpret(&mut self, expression: &Expression) {
        let value = self.evaluate(expression);

        match value {
            std::result::Result::Ok(value) => println!("{}", self.stringify(value)),
            Err(error) => JSPlusPlus::error(0, &error.to_string()),
        }
    }

    fn stringify(&self, value: Box<dyn Any>) -> String {
        match value.downcast_ref::<f64>() {
            Some(value) => {
                let text = value.to_string();

                if text.ends_with(".0") {
                    text[..text.len() - 2].to_string()
                } else {
                    text
                }
            }
            None => match value.downcast_ref::<bool>() {
                Some(value) => value.to_string(),
                None => match value.downcast_ref::<String>() {
                    Some(value) => value.to_string(),
                    None => "nil".to_string(),
                },
            },
        }
    }

    fn evaluate(&mut self, expression: &Expression) -> Result<Box<dyn Any>> {
        Ok(expression.accept(self))
    }

    fn is_truthy(&self, value: &Box<dyn Any>) -> bool {
        match value.downcast_ref::<bool>() {
            Some(value) => *value,
            None => false,
        }
    }

    fn is_equal(&self, left: &Box<dyn Any>, right: &Box<dyn Any>) -> bool {
        match (left.downcast_ref::<bool>(), right.downcast_ref::<bool>()) {
            (Some(left), Some(right)) => left == right,
            _ => false,
        }
    }

    fn check_number_operand(&self, operator: &Token, operand: &Box<dyn Any>) -> Result<()> {
        if operand.is::<f64>() {
            return Ok(());
        }

        bail!(RuntimeError::new(
            operator.clone(),
            "Operand must be a number."
        ));
    }

    fn check_number_operands(
        &self,
        operator: &Token,
        left: &Box<dyn Any>,
        right: &Box<dyn Any>,
    ) -> Result<()> {
        if left.is::<f64>() && right.is::<f64>() {
            return Ok(());
        }

        let message = format!(
            "Operands must be numbers. Got {:?} and {:?} instead.",
            left, right
        );

        // Err(RuntimeError::new(
        //     operator.clone(),
        //     Into::<std::str::Split<'_, &str>>::into(
        //         message.as_str().split("").into_iter().copied(),
        //     )
        //     .,
        // ))
        Ok(())
    }
}

impl ExpressionVisitor<Box<dyn Any>> for Interpreter {
    fn visit_assign_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_binary_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        match expr {
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                let left = left.accept(self);
                let right = right.accept(self);

                match operator.token_type {
                    TokenType::Minus => {
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left - right)
                    }
                    TokenType::Plus => {
                        if left.is::<String>() || right.is::<String>() {
                            let left = left.downcast_ref::<String>().unwrap();
                            let right = right.downcast_ref::<String>().unwrap();
                            Box::new(format!("{}{}", left, right))
                        } else {
                            let left = left.downcast_ref::<f64>().unwrap();
                            let right = right.downcast_ref::<f64>().unwrap();
                            Box::new(left + right)
                        }
                    }
                    TokenType::Slash => {
                        if let Err(err) = self.check_number_operands(&operator, &left, &right) {
                            panic!("{}", err);
                        }
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left / right)
                    }
                    TokenType::Star => {
                        if let Err(err) = self.check_number_operands(&operator, &left, &right) {
                            panic!("{}", err);
                        }
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left * right)
                    }
                    TokenType::Greater => {
                        if let Err(err) = self.check_number_operands(&operator, &left, &right) {
                            panic!("{}", err);
                        }
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left > right)
                    }
                    TokenType::GreaterEqual => {
                        if let Err(err) = self.check_number_operands(&operator, &left, &right) {
                            panic!("{}", err);
                        }
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left >= right)
                    }
                    TokenType::Less => {
                        if let Err(err) = self.check_number_operands(&operator, &left, &right) {
                            panic!("{}", err);
                        }
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left < right)
                    }
                    TokenType::LessEqual => {
                        if let Err(err) = self.check_number_operands(&operator, &left, &right) {
                            panic!("{}", err);
                        }
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left <= right)
                    }
                    TokenType::BangEqual => Box::new(!self.is_equal(&left, &right)),
                    TokenType::EqualEqual => Box::new(self.is_equal(&left, &right)),
                    _ => Box::new(()),
                }
            }
            _ => panic!("Expected binary expression"),
        }
    }

    fn visit_call_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_get_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_grouping_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        match expr {
            Expression::Grouping { expression: _ } => expr.accept(self),
            _ => panic!("Expected grouping expression"),
        }
    }

    fn visit_literal_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        match expr {
            Expression::Literal { value } => Box::new(value.clone()),
            _ => panic!("Expected literal expression"),
        }
    }

    fn visit_logical_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_set_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_unary_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        match expr {
            Expression::Unary { operator, right } => {
                let right = right.accept(self);

                match operator.token_type {
                    TokenType::Minus => {
                        if let Err(err) = self.check_number_operand(&operator, &right) {
                            panic!("{}", err);
                        }
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(-right)
                    }
                    TokenType::Bang => Box::new(!self.is_truthy(&right)),
                    _ => Box::new(()),
                }
            }
            _ => panic!("Expected unary expression"),
        }
    }

    fn visit_variable_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }
}
