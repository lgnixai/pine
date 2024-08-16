use crate::lexer::ast::Statement;

#[derive(Debug, Clone)]
pub struct Module {
    pub stmt: Statement,
}
