use tsr_parser::input::Positioned;
use tsr_parser::lexer::ast::IfStatement;
use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_if(&mut self, statement: Positioned<IfStatement>) -> Value {
        let (span, ifstate) = statement.unpack();

        let if_statement = ifstate.expression;
        let condition_value = self.eval_expression(if_statement);
        println!("condition_value{:?}",condition_value);
        println!("ifstate.then_statement{:?}",ifstate.then_statement);
        if let Value::Boolean(true) = condition_value {
            //self.eval_block(ifstate.then_statement)
             self.eval_statement(ifstate.then_statement);

        } else if let Some(else_statement) = ifstate.else_statement {
            //self.eval_block(else_statement)
            self.eval_statement(else_statement);

        } else {
           //Value::None
        }
        // if condition_value {
        //     self.eval_block(ifstate.then_statement)
        // } else if let Some(else_statement) = ifstate.else_statement {
        //     self.eval_block(else_statement)
        // } else {
        //     Value::None
        // }

        Value::None
    }
}
