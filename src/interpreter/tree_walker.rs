use std::str::FromStr;

use super::value::LiteralValue;
use crate::parser::ast::*;
use crate::parser::visitor::Visitor;
use crate::token::{Token, TokenType};

pub struct RuntimeError {
    pub message: String,
    pub token: Token,
}
struct TreeWalker;

pub struct Interpreter {
    tree_walker: TreeWalker,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            tree_walker: TreeWalker,
        }
    }
    pub fn interpret(&self, expr: &Expression) -> Result<LiteralValue, RuntimeError> {
        self.tree_walker.visit_expression(expr)
    }
}

impl Visitor<Result<LiteralValue, RuntimeError>> for TreeWalker {
    fn visit_binary_expression(
        &self,
        expr: &BinaryExpression,
    ) -> Result<LiteralValue, RuntimeError> {
        let left_val = self.visit_expression(&expr.left)?;
        let right_val = self.visit_expression(&expr.right)?;
        match expr.operator.r#type {
            TokenType::Minus => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Number(left_num - right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::Slash => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Number(left_num / right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::Star => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Number(left_num * right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::Plus => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Number(left_num + right_num))
                }
                (LiteralValue::String(left_str), LiteralValue::String(right_str)) => {
                    return Ok(LiteralValue::String(left_str + &right_str))
                }
                _ => Err(RuntimeError {
                    message: "operands must be two numbers or two strings.".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::Greater => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Bool(left_num > right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::GreaterEqual => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Bool(left_num >= right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::Less => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Bool(left_num < right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::LessEqual => match (left_val, right_val) {
                (LiteralValue::Number(left_num), LiteralValue::Number(right_num)) => {
                    return Ok(LiteralValue::Bool(left_num <= right_num))
                }
                _ => Err(RuntimeError {
                    message: "invalid operand type: a numerical value is expected".to_string(),
                    token: expr.operator.clone(),
                }),
            },
            TokenType::EqualEqual => return Ok(LiteralValue::Bool(left_val == right_val)),
            TokenType::BangEqual => return Ok(LiteralValue::Bool(!(left_val == right_val))),
            _ => Err(RuntimeError {
                message: "operator not supported".to_string(),
                token: expr.operator.clone(),
            }),
        }
    }

    fn visit_grouping_expression(
        &self,
        expr: &GroupingExpression,
    ) -> Result<LiteralValue, RuntimeError> {
        self.visit_expression(&expr.0)
    }

    fn visit_literal_expression(
        &self,
        expr: &LiteralExpression,
    ) -> Result<LiteralValue, RuntimeError> {
        match expr {
            LiteralExpression::Nil(_) => Ok(LiteralValue::Nil),
            LiteralExpression::Bool(token) => Ok(LiteralValue::Bool(
                FromStr::from_str(&token.lexeme).unwrap(),
            )),
            LiteralExpression::String(token) => Ok(LiteralValue::String(
                token.lexeme.trim_matches('"').to_string(),
            )),
            LiteralExpression::Number(token) => {
                Ok(LiteralValue::Number(f64::from_str(&token.lexeme).unwrap()))
            }
        }
    }

    fn visit_unary_expression(&self, expr: &UnaryExpression) -> Result<LiteralValue, RuntimeError> {
        match expr.operator.r#type {
            TokenType::Minus => {
                if let LiteralValue::Number(number) = self.visit_expression(&expr.operand)? {
                    Ok(LiteralValue::Number(-number))
                } else {
                    Err(RuntimeError {
                        message: "invalid operand type: a numerical value is expected".to_string(),
                        token: expr.operator.clone(),
                    })
                }
            }
            TokenType::Bang => Ok(LiteralValue::Bool(
                !self.visit_expression(&expr.operand)?.is_truthy(),
            )),
            _ => Err(RuntimeError {
                message: "invalid operand type: a numerical value is expected".to_string(),
                token: expr.operator.clone(),
            }),
        }
    }
}
