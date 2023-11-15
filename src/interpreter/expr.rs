use crate::{
    lexer::tokens::{Token, TokenType},
    parser::expr::{BinaryNode, Expression, ExpressionNode, PrimaryNode, UnaryNode},
};

use super::value::Value;

pub trait Evaluatable {
    fn eval(&self) -> Result<Value, ()>;
}

impl Evaluatable for Expression {
    fn eval(&self) -> Result<Value, ()> {
        self.get_node().eval()
    }
}

impl Evaluatable for ExpressionNode {
    fn eval(&self) -> Result<Value, ()> {
        match self {
            ExpressionNode::Primary(primary) => primary.eval(),
            ExpressionNode::Unary(unary) => unary.eval(),
            ExpressionNode::Binary(binary) => binary.eval(),
        }
    }
}

impl Evaluatable for BinaryNode {
    fn eval(&self) -> Result<Value, ()> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;
        let val = match self.op.typ {
            TokenType::Minus => left - right,
            TokenType::Plus => left + right,
            TokenType::Dash => left / right,
            TokenType::Star => left * right,
            _ => unreachable!(),
        };

        match val {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        }
    }
}

impl Evaluatable for UnaryNode {
    fn eval(&self) -> Result<Value, ()> {
        let expr = self.expr.eval()?;
        let val = match self.op.typ {
            TokenType::Minus => -expr,
            _ => unreachable!(),
        };

        match val {
            Ok(v) => Ok(v),
            Err(_) => Err(()),
        }
    }
}

impl Evaluatable for PrimaryNode {
    fn eval(&self) -> Result<Value, ()> {
        match self {
            PrimaryNode::Literal(token) => eval_literal(token),
            _ => unreachable!(),
        }
    }
}

fn eval_literal(token: &Token) -> Result<Value, ()> {
    let value = match token.typ.clone() {
        TokenType::Int(i) => Value::Int(i),
        _ => unreachable!(),
    };

    Ok(value)
}
