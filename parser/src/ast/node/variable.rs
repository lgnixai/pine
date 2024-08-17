use crate::ast::types::types::{DeclarationMode};
use crate::input::Positioned;
use crate::lexer::ast::{Expression, Type};
use crate::lexer::identifier::Identifier;

#[derive(PartialEq, Debug, Clone)]
pub struct VariableStatement {
    pub mutable: Positioned<bool>,
    pub declarations: Vec<Positioned<VariableDeclaration>>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDeclaration {
    pub declaration_mode: Option<Positioned<DeclarationMode>>,

    pub name: Positioned<Identifier>,
    pub ty: Option<Positioned<Type>>,
    //pub nullable: Positioned<bool>,
    pub initializer: Option<Positioned<Expression>>,


}