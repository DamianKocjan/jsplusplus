use std::any::Any;
use std::collections::HashMap;

use anyhow::{bail, Result};

use crate::parser::{
    expression::{Expression, Visitor as ExpressionVisitor},
    token::TokenType,
};

pub struct Interpreter;

impl Interpreter {
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

    fn check_number_operand(&self, operator: &TokenType, operand: &Box<dyn Any>) {
        if operand.is::<f64>() {
            return;
        }

        panic!("Operand must be a number. Got {:?} instead.", operator);
    }

    fn check_number_operands(
        &self,
        operator: &TokenType,
        left: &Box<dyn Any>,
        right: &Box<dyn Any>,
    ) {
        if left.is::<f64>() && right.is::<f64>() {
            return;
        }

        panic!(
            "Operands must be numbers. Got {:?} and {:?} instead.",
            left, right
        );
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
                        self.check_number_operands(&operator.token_type, &left, &right);
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left / right)
                    }
                    TokenType::Star => {
                        self.check_number_operands(&operator.token_type, &left, &right);
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left * right)
                    }
                    TokenType::Greater => {
                        self.check_number_operands(&operator.token_type, &left, &right);
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left > right)
                    }
                    TokenType::GreaterEqual => {
                        self.check_number_operands(&operator.token_type, &left, &right);
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left >= right)
                    }
                    TokenType::Less => {
                        self.check_number_operands(&operator.token_type, &left, &right);
                        let left = left.downcast_ref::<f64>().unwrap();
                        let right = right.downcast_ref::<f64>().unwrap();
                        Box::new(left < right)
                    }
                    TokenType::LessEqual => {
                        self.check_number_operands(&operator.token_type, &left, &right);
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

    fn visit_super_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_this_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        todo!()
    }

    fn visit_unary_expression(&mut self, expr: &Expression) -> Box<dyn Any> {
        match expr {
            Expression::Unary { operator, right } => {
                let right = right.accept(self);

                match operator.token_type {
                    TokenType::Minus => {
                        self.check_number_operand(&operator.token_type, &right);
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
