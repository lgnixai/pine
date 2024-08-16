use tsr_parser::lexer::ast::Literal;
use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_literal(&self, literal: Literal) -> Value {
        match literal {
            Literal::String(string) => Value::String(string.value),
            Literal::Number(number) => Value::Number(number.value),
            Literal::Float(float) => Value::Float(float.value),
            Literal::Boolean(boolean) => Value::Boolean(boolean.value),
            _ => {
                Value::String("asdf".to_string())
            }
        }
    }
}