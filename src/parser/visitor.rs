use std::cell::RefCell;
use std::fs::File;
use std::io::{self, Write};

use graphviz_rust::dot_generator::{attr, edge, graph, id, node, node_id};
use graphviz_rust::dot_structures::{
    Attribute, Edge, EdgeTy, Graph, Id, Node, NodeId, Stmt, Vertex,
};
use graphviz_rust::printer::PrinterContext;
use graphviz_rust::{cmd::Format, exec};

use crate::parser::ast::*;

pub trait Visitor<T> {
    fn visit_expression(&self, expr: &Expression) -> T {
        match expr {
            Expression::Binary(binary_expression) => {
                Self::visit_binary_expression(self, binary_expression)
            }
            Expression::Grouping(grouping_expression) => {
                Self::visit_grouping_expression(self, grouping_expression)
            }
            Expression::Literal(literal_expression) => {
                Self::visit_literal_expression(self, literal_expression)
            }
            Expression::Unary(unary_expression) => {
                Self::visit_unary_expression(self, unary_expression)
            }
        }
    }
    fn visit_binary_expression(&self, expr: &BinaryExpression) -> T;
    fn visit_grouping_expression(&self, expr: &GroupingExpression) -> T;
    fn visit_literal_expression(&self, expr: &LiteralExpression) -> T;
    fn visit_unary_expression(&self, expr: &UnaryExpression) -> T;
}

pub struct PrettyPrinter;

impl Visitor<String> for PrettyPrinter {
    fn visit_binary_expression(&self, expr: &BinaryExpression) -> String {
        format!(
            "({} {} {})",
            expr.operator.lexeme,
            Self::visit_expression(self, &expr.left),
            Self::visit_expression(self, &expr.right),
        )
    }
    fn visit_grouping_expression(&self, expr: &GroupingExpression) -> String {
        format!("({})", Self::visit_expression(self, &expr.0))
    }
    fn visit_literal_expression(&self, expr: &LiteralExpression) -> String {
        match expr {
            LiteralExpression::Bool(token) => token.lexeme.clone(),
            LiteralExpression::Nil(token) => token.lexeme.clone(),
            LiteralExpression::Number(token) => token.lexeme.clone(),
            LiteralExpression::String(token) => token.lexeme.clone(),
        }
    }
    fn visit_unary_expression(&self, expr: &UnaryExpression) -> String {
        format!(
            "({} {})",
            expr.operator.lexeme,
            Self::visit_expression(self, &expr.operand)
        )
    }
}

pub struct GraphGenerator {
    sequence: RefCell<u64>,
    graph: RefCell<Graph>,
}

impl GraphGenerator {
    pub fn new() -> Self {
        GraphGenerator {
            sequence: RefCell::new(0),
            graph: RefCell::new(graph!(strict di id!("parser_tree"))),
        }
    }

    pub fn generate_tree(&self, expr: &Expression) -> std::io::Result<()> {
        self.visit_expression(expr);

        let graph_svg = exec(
            self.graph.borrow_mut().clone(),
            &mut PrinterContext::default(),
            vec![Format::Svg.into()],
        )?;

        let mut file = File::create("./parse_tree.svg")?;
        file.write_all(graph_svg.as_bytes())?;

        Ok(())
    }
}

impl Visitor<()> for GraphGenerator {
    fn visit_binary_expression(&self, expr: &BinaryExpression) -> () {
        self.visit_expression(&expr.left);

        // the id of the previous lef
        let left_node_id = *self.sequence.borrow() - 1;

        self.visit_expression(&expr.right);
        let right_node_id = *self.sequence.borrow() - 1;

        let operator_id = right_node_id + 1;
        let operator_node =
            node!(operator_id.to_string();attr!("label",(format!("\"{}\"", expr.operator.lexeme))));

        self.graph.borrow_mut().add_stmt(Stmt::Node(operator_node));
        self.graph.borrow_mut().add_stmt(Stmt::Edge(
            edge!(node_id!(operator_id) => node_id!(left_node_id)),
        ));
        self.graph.borrow_mut().add_stmt(Stmt::Edge(
            edge!(node_id!(operator_id) => node_id!(right_node_id)),
        ));

        *self.sequence.borrow_mut() += 1;
    }

    fn visit_grouping_expression(&self, expr: &GroupingExpression) -> () {
        self.visit_expression(&expr.0);
    }

    fn visit_literal_expression(&self, expr: &LiteralExpression) -> () {
        let literal_id = *self.sequence.borrow();
        match expr {
            LiteralExpression::Bool(token)
            | LiteralExpression::Nil(token)
            | LiteralExpression::Number(token)
            | LiteralExpression::String(token) => {
                let literal_node =
                    node!(literal_id.to_string();attr!("label",(format!("\"{}\"", token.lexeme))));
                self.graph.borrow_mut().add_stmt(Stmt::Node(literal_node));
            }
        }
        *self.sequence.borrow_mut() += 1;
    }

    fn visit_unary_expression(&self, expr: &UnaryExpression) -> () {
        self.visit_expression(&expr.operand);
        let operator_id = *self.sequence.borrow();
        let expression_id = operator_id - 1;

        let operator_node =
            node!(operator_id.to_string();attr!("label",(format!("\"{}\"", expr.operator.lexeme))));

        self.graph.borrow_mut().add_stmt(Stmt::Node(operator_node));
        self.graph.borrow_mut().add_stmt(Stmt::Edge(
            edge!(node_id!(operator_id) => node_id!(expression_id)),
        ));
        *self.sequence.borrow_mut() += 1;
    }
}
