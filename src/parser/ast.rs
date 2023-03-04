use crate::token::Token;

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpression(pub Box<Expression>);

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Token,
    pub operand: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
    Literal(LiteralExpression),
    Unary(UnaryExpression),
}

#[derive(Debug, Clone)]
pub enum LiteralExpression {
    Bool(Token),
    Nil(Token),
    Number(Token),
    String(Token),
}

impl Expression {
    fn binary(left: Expression, operator: Token, right: Expression) -> Self {
        Self::Binary(BinaryExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn grouping(expr: Expression) -> Self {
        Self::Grouping(GroupingExpression(Box::new(expr)))
    }

    fn unary(operator: Token, operand: Expression) -> Self {
        Self::Unary(UnaryExpression {
            operator,
            operand: Box::new(operand),
        })
    }

    fn bool(token: Token) -> Self {
        Self::Literal(LiteralExpression::Bool(token))
    }

    fn nil(token: Token) -> Self {
        Self::Literal(LiteralExpression::Nil(token))
    }

    fn number(token: Token) -> Self {
        Self::Literal(LiteralExpression::Number(token))
    }

    fn string(token: Token) -> Self {
        Self::Literal(LiteralExpression::String(token))
    }
}
