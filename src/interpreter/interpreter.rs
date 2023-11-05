use super::stmt::Executable;
use crate::parser::stmt::Statement;

pub struct Interpreter {
    statements: Vec<Statement>,
}

impl Interpreter {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn interpret(&self) {
        for stmt in &self.statements {
            let evaluated = stmt.run();
            println!("{:#?}", evaluated);
        }
    }
}
