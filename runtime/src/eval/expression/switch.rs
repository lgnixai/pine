// use tsr_parser::ast::MatchExpression;
//
// use crate::{value::Value, Runtime};
//
// impl Runtime {
//
//
//     pub fn eval_match_expression(&mut self, expression: MatchExpression) -> Value {
//         let target = self.eval_expression(expression.target);
//         println!("match,{:?}",target);
//         let target = self.resolve_if_reference(target);
//         println!("match,{:?}",target);
//
//         for variant in expression.variants {
//             let value = self.eval_expression(variant.value.value);
//             println!("expression.variants,{:?}",value);
//
//             if target == value {
//                 return self.eval_statement(variant.value.callback);
//             }
//         }
//
//         Value::None
//     }
// }