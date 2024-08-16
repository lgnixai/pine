use tsr_parser::lexer::ast::BinaryExpression;
use tsr_parser::lexer::token::Operator;
use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_binary_expression(&mut self, expression: BinaryExpression) -> Value {
        // let left = self.eval_expression(expression.left);
        // let right = self.eval_expression(expression.right);

        let left = self.eval_expression(expression.left);
        let right = self.eval_expression(expression.right);

        let left = self.resolve_if_reference(left);
        let right = self.resolve_if_reference(right);

        println!("left{left} op {:?}right {right}", expression.operator.value);
        //println!("{left} {:?} {right}", expression.operator.value);

        match expression.operator.value {
            Operator::And => todo!(),
            Operator::AndAnd => todo!(),
            Operator::Plus => match (left, right) {
                // (Value::String(first), Value::String(second)) => Value::String(first + &second),
                // (Value::Number(first), Value::Number(second)) => Value::Number(first + second),

                (Value::String(first), Value::String(second)) => Value::String(first + &second),
                (Value::Number(first), Value::Number(second)) => Value::Number(first + second),
                (Value::Reference(path, scope), Value::Number(second)) => {
                    match self.resolve_reference(&path, scope) {
                        Ok(value) => match value {
                            Value::Number(first) => Value::Number(first + second),
                            _ => Value::None
                        },
                        Err(e) => Value::None,
                    }
                }
                (Value::Number(first), Value::Reference(path, scope), ) => {
                    match self.resolve_reference(&path, scope) {
                        Ok(value) => match value {
                            Value::Number(second) => Value::Number(first + second),
                            _ => Value::None
                        },
                        Err(e) => Value::None,
                    }
                }
                (Value::Reference(left_path, left_scope), Value::Reference(right_path, right_scope)) => {
                    match (self.resolve_reference(&left_path, left_scope), self.resolve_reference(&right_path, right_scope)) {
                        (Ok(left_value), Ok(right_value)) => match (left_value, right_value) {
                            (Value::Number(first), Value::Number(second)) => Value::Number(first + second),
                            (Value::String(first), Value::String(second)) => Value::String(first + &second),
                            // 可以根据需要添加其他类型的组合
                            _ => Value::None // 或者返回一个错误
                        },
                        _ => Value::None // 如果任何一个引用解析失败，返回 None 或错误
                    }
                }
                (_, _) => todo!(),
            },
            Operator::Star => match (left, right) {
                (Value::String(data), Value::Number(times)) => {
                    Value::String(data.repeat(times as usize))
                }
                (Value::Number(first), Value::Number(second)) => Value::Number(first * second),
                (_, _) => todo!(),
            },
            Operator::Slash => match (left, right) {
                (Value::Number(first), Value::Number(second)) => Value::Number(first / second),
                (_, _) => todo!(),
            },
            Operator::Or => todo!(),
            Operator::OrOr => todo!(),
            Operator::PlusPlus => match left {
                Value::Reference(path, scope) => {
                    let mut context = self.context.lock().unwrap();

                    if let Some(Value::Number(value)) =
                        context.get(&path[0], scope.clone()).map(|var| &var.value)
                    {
                        let value = Value::Number(value + 1);

                        context.set(&path, scope, value);
                    }

                    Value::None
                }
                _ => todo!(),
            },
            Operator::Minus => match (left, right) {
                (Value::Number(first), Value::Number(second)) => Value::Number(first - second),
                (_, _) => todo!(),
            },
            Operator::MinusMinus => match left {
                Value::Reference(path, scope) => {
                    let mut context = self.context.lock().unwrap();

                    if let Some(Value::Number(value)) =
                        context.get(&path[0], scope.clone()).map(|var| &var.value)
                    {
                        let value = Value::Number(value - 1);

                        context.set(&path, scope, value);
                    }

                    Value::None
                }
                _ => todo!(),
            },
            Operator::EqEq => Value::Boolean(left == right),
            Operator::Eq => match (left, right) {
                (Value::Reference(path, scope), value) => {
                    let value = match value {
                        Value::Reference(path, scope) => todo!(),
                        value => value,
                    };

                    self.context.lock().unwrap().set(&path, scope, value);

                    Value::None
                }
                (_, _) => {
                    // let left = self.resolve_if_reference(left);
                    // let left_clone = left.clone();
                    // let right_clone = right.clone();
                    //println!("{:?}:{:?}", left_clone, right_clone);
                    //println!("{:?}:{:?}",left.clone(),right.clone());
                    Value::None
                }
            },
            Operator::Ne => Value::Boolean(left != right),
            Operator::Le => todo!(),
            Operator::Ge => todo!(),
            Operator::Lt => todo!(),
            Operator::Gt => todo!(),
            Operator::Not => match left {
                Value::Reference(path, scope) => {
                    if let Some(Value::Boolean(value)) = self
                        .context
                        .lock()
                        .unwrap()
                        .get(&path[0], scope.clone())
                        .map(|var| &var.value)
                    {
                        return Value::Boolean(!value);
                    }

                    Value::None
                }
                Value::Boolean(value) => Value::Boolean(!value),
                _ => todo!(),
            },
        }
    }
}