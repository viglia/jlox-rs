pub(crate) mod ast;
mod parser;
pub mod visitor;

use crate::token::Token;
use parser::Parser;

pub fn parse(tokens: Vec<Token>) -> ast::Expression {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
