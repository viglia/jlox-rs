use std::collections::btree_map::Iter;

use multipeek::{multipeek, MultiPeek};

use crate::token::{Token, TokenType};

use super::ast::{
    BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression,
};

pub struct Parser {
    tokens: MultiPeek<std::vec::IntoIter<Token>>,
    current: u64,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: multipeek(tokens.into_iter()),
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Expression {
        self.expression()
    }

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();

        loop {
            match self.tokens.peek().unwrap().r#type {
                TokenType::BangEqual | TokenType::EqualEqual => {
                    let operator = self.tokens.next().unwrap();
                    let right = self.comparison();
                    expr = Expression::Binary(BinaryExpression {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    });
                    continue;
                }
                _ => {}
            }
            break;
        }

        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        loop {
            match self.tokens.peek().unwrap().r#type {
                TokenType::Less
                | TokenType::LessEqual
                | TokenType::Greater
                | TokenType::GreaterEqual => {
                    let operator = self.tokens.next().unwrap();
                    let right = self.term();
                    expr = Expression::Binary(BinaryExpression {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    });
                    continue;
                }
                _ => {}
            }
            break;
        }
        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        loop {
            match self.tokens.peek().unwrap().r#type {
                TokenType::Minus | TokenType::Plus => {
                    let operator = self.tokens.next().unwrap();
                    let right = self.factor();
                    expr = Expression::Binary(BinaryExpression {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    });
                    continue;
                }
                _ => {}
            }
            break;
        }
        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        loop {
            match self.tokens.peek().unwrap().r#type {
                TokenType::Slash | TokenType::Star => {
                    let operator = self.tokens.next().unwrap();
                    let right = self.unary();
                    expr = Expression::Binary(BinaryExpression {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    });
                    continue;
                }
                _ => {}
            }
            break;
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        match self.tokens.peek().unwrap().r#type {
            TokenType::Bang | TokenType::Minus => Expression::Unary(UnaryExpression {
                operator: self.tokens.next().unwrap(),
                operand: Box::new(self.unary()),
            }),
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Expression {
        match self.tokens.peek().unwrap().r#type {
            TokenType::True | TokenType::False => {
                Expression::Literal(LiteralExpression::Bool(self.tokens.next().unwrap()))
            }
            TokenType::Nil => {
                Expression::Literal(LiteralExpression::Nil(self.tokens.next().unwrap()))
            }
            TokenType::String(_) => {
                Expression::Literal(LiteralExpression::String(self.tokens.next().unwrap()))
            }
            TokenType::Number(_) => {
                Expression::Literal(LiteralExpression::Number(self.tokens.next().unwrap()))
            }
            TokenType::LeftParen => {
                // consume the left parethesis
                self.tokens.next();
                let expr = self.expression();
                // temporary panic
                // TODO: substitute with proper error handling
                if self.tokens.peek().unwrap().r#type != TokenType::RightParen {
                    panic!("Error: expected ')' after expression")
                }
                // consume the right parethesis
                self.tokens.next();
                Expression::Grouping(GroupingExpression(Box::new(expr)))
            }
            _ => panic!("Error: token not expected"),
        }
    }
}
