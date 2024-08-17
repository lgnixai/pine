#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Color,
    String,
    Line,
    LineFill,
    Label,
    Box,
    Table,
    Array(Box<Type>),
    Matrix(Box<Type>),
    UDF,
    // 其他类型
}


#[derive(Debug, Clone, PartialEq)]
pub enum DeclarationMode {
    Var,
    Varip,
    Const,
}
