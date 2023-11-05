use super::{executable::Executable, expr::Evaluatable, value::Value};
use crate::parser::stmt::{ExprStatement, Statement};

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
