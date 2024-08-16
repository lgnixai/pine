use tsr_parser::lexer::ast::{ArraySize, IndexExpression, PredefinedType, PrimaryType};

use crate::{value::{ErrorCode, Value}, FunctionBuilder, Runtime};

impl Runtime {
    pub fn eval_index_expression(&mut self, expression: IndexExpression) -> Value {
        let (target_span, target) = (
            expression.target.span,
            self.eval_expression(expression.target),
        );
        let (index_span, index) = (
            expression.index.span,
            self.eval_expression(expression.index),
        );

        println!("trying to access {index:?} inside {target:?}");

        match (target, index) {
            (Value::Array(elements, _), Value::Number(index)) => {
                elements.get(index as usize).unwrap_or(&Value::None).clone()
            }
            (Value::Array(..), Value::String(_)) => todo!(),
            (Value::Object(properties), index) => {
                properties.get(&index).unwrap_or(&Value::None).clone()
            }
            (Value::String(string), Value::Number(index)) => Value::String(
                string
                    .get(index as usize..=index as usize)
                    .unwrap_or_default()
                    .to_string(),
            ),
            (Value::String(string), Value::String(name)) => match name.as_str() {
                "length" => Value::Number(string.len() as i64),
                "startsWith" => FunctionBuilder::new("startsWith")
                    .param("data", PredefinedType::String)
                    .returns(PredefinedType::Boolean)
                    .build(move |args| {
                        args.returns(Value::Boolean(string.starts_with(&args.get_string("data"))));
                    }),
                "split" => FunctionBuilder::new("split")
                    .param("pattern", PredefinedType::String)
                    .returns(PrimaryType::ArrayType(
                        Box::new(PredefinedType::String.into()),
                        ArraySize::Dynamic,
                    ))
                    .build(move |args| {
                        args.returns(Value::Array(
                            string
                                .split(&args.get_string("pattern"))
                                .map(|part| Value::String(part.into()))
                                .collect(),
                            ArraySize::Dynamic,
                        ));
                    }),
                &_ => Value::error(target_span, ErrorCode::Implementing, "implementing"),
            },
            (Value::ReturnValue(value), _) => *value,
            (Value::Enum(enumeration), Value::String(key)) => enumeration
                .get_moved(key)
                .map(|member| *member.init)
                .unwrap_or(Value::error(
                    target_span,
                    ErrorCode::Implementing,
                    "no such field",
                )),
            (Value::Null, _) => {
                Value::error(target_span, ErrorCode::Reference, "can't index null value")
            }
            (Value::ClassInstance(instance), Value::String(key)) => instance
                .get_field_moved(key)
                .map_or(Value::None, |field| field.value),
            // (Value::Reference(path, scope), Value::Reference(second_path, _)) => {
            //     if second_path[0] == "push"
            //         && todo!()
            //     {
            //         Value::None
            //     } else {
            //         Value::Reference([path, second_path].concat(), scope)
            //     }
            // }


            (Value::Reference(path, scope), Value::Number(index)) => {
                match self.resolve_reference(&path, scope) {
                    Ok(value) => match value {
                        Value::Array(elements, _) =>
                            elements.get(index as usize).unwrap_or(&Value::None).clone(),

                        _ => Value::None
                    },
                    Err(e) => Value::None,
                }
            }

            (Value::Reference(left_path, left_scope), Value::Reference(right_path, right_scope)) => {
                let key1 = right_path.last();
                let key = (key1.unwrap());

                println!("=={:?}", right_path.last());
                match self.resolve_reference(&left_path, left_scope) {
                    (Ok(left_value)) => match (left_value) {
                        (Value::Enum(enumeration)) => enumeration
                            .get_moved(key)
                            .map(|member| *member.init)
                            .unwrap_or(Value::error(
                                target_span,
                                ErrorCode::Implementing,
                                "no such field",
                            )),
                        (Value::Object(properties)) => {
                            properties.get(&Value::String(key.to_string())).unwrap_or(&Value::None).clone()
                        }

                        // 可以根据需要添加其他类型的组合
                        _ => Value::None // 或者返回一个错误
                    },
                    _ => Value::None // 如果任何一个引用解析失败，返回 None 或错误
                }
            }
            _ => Value::error(target_span, ErrorCode::Reference, "can't index"),
        }
    }
}