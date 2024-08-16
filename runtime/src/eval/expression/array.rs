use tsr_parser::input::Positioned;
use tsr_parser::lexer::ast::{ArraySize, Expression};

use crate::{value::Value, Runtime};
use tsr_parser::lexer::util::VecExt;
use tsr_parser::lexer::util::BoolExt;
impl Runtime {
    pub fn eval_array(
        &mut self,
        elements: Vec<Positioned<Expression>>,
        is_dynamic: Positioned<bool>,
    ) -> Value {
        let elements = elements
            .into_iter()
            .map(|expression| match self.eval_expression(expression) {
                Value::Reference(path, scope) => {

                    //println!("=={:?},{:?}",path,scope);
                    Value::None
                },
                value => value,
            })
            .collect::<Vec<_>>();
        let size = elements.len();

        Value::Array(
            elements,
            is_dynamic
                .value
                .map(ArraySize::Dynamic, ArraySize::Fixed(size)),
        )
    }
}
