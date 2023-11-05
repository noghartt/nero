use crate::lexer::tokens::{Token, TokenType};

use super::token_stream::TokenStream;

#[derive(Debug)]
pub struct Expression {
    node: ExpressionNode,
}

impl Expression {
    pub fn new(node: ExpressionNode) -> Box<Self> {
        Box::new(Self { node })
    }

    pub fn get_node(&self) -> &ExpressionNode {
        &self.node
    }
}

#[derive(Debug)]
pub enum ExpressionNode {
    Primary(PrimaryNode),
    Unary(UnaryNode),
    Binary(BinaryNode),
}

#[derive(Debug)]
pub enum PrimaryNode {
    Literal(Token),
}

#[derive(Debug)]
pub struct UnaryNode {
    pub expr: Box<Expression>,
    pub op: Token,
}

#[derive(Debug)]
pub struct BinaryNode {
    pub left: Box<Expression>,
    pub op: Token,
    pub right: Box<Expression>,
}

pub fn expression(tokens: &mut TokenStream) -> Result<Box<Expression>, ()> {
    term(tokens)
}

fn term(tokens: &mut TokenStream) -> Result<Box<Expression>, ()> {
    let mut expr = unary(tokens);
    while tokens.match_next(&[TokenType::Minus, TokenType::Plus]) {
        let node = ExpressionNode::Binary(BinaryNode {
            left: expr?,
            op: tokens.prev().clone(),
            right: expression(tokens)?,
        });

        expr = Ok(Expression::new(node));
    }

    return expr;
}

fn unary(tokens: &mut TokenStream) -> Result<Box<Expression>, ()> {
    if tokens.match_next(&[TokenType::Minus]) {
        let node = ExpressionNode::Unary(UnaryNode {
            op: tokens.prev().clone(),
            expr: primary(tokens)?,
        });

        return Ok(Expression::new(node));
    }

    primary(tokens)
}

fn primary(tokens: &mut TokenStream) -> Result<Box<Expression>, ()> {
    let node = ExpressionNode::Primary(match &tokens.accept().typ {
        TokenType::Int(_) => PrimaryNode::Literal(tokens.prev().clone()),
        _ => return Err(()),
    });
    Ok(Expression::new(node))
}
