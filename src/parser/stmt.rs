use super::{
    expr::{expression, Expression},
    token_stream::TokenStream,
};

#[derive(Debug)]
pub enum Statement {
    Expr(ExprStatement),
}

#[derive(Debug)]
pub struct ExprStatement {
    pub expr: Box<Expression>,
}

// TODO: Implement Error on parsing statements
pub fn statement(tokens: &mut TokenStream) -> Result<Statement, ()> {
    let token = tokens.accept();
    let stmt = match token.typ {
        _ => {
            tokens.discard();
            Statement::Expr(parse_expr(tokens))
        }
    };
    Ok(stmt)
}

fn parse_expr(tokens: &mut TokenStream) -> ExprStatement {
    let Ok(expr) = expression(tokens) else {
        panic!("Wrong parsing");
    };
    ExprStatement { expr }
}
