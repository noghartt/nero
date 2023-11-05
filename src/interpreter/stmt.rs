use super::{expr::Evaluatable, value::Value};
use crate::parser::stmt::{ExprStatement, Statement};

pub trait Executable {
    fn run(&self) -> Result<Value, ()>;
}

impl Executable for Statement {
    fn run(&self) -> Result<Value, ()> {
        match self {
            Statement::Expr(expr) => expr.run(),
        }
    }
}

impl Executable for ExprStatement {
    fn run(&self) -> Result<Value, ()> {
        match self.expr.eval() {
            Ok(v) => Ok(v),
            Err(e) => Err(e),
        }
    }
}
