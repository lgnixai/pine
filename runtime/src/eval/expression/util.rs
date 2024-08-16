
use crate::{value::Value, Runtime};
use crate::environment::Scope;
use crate::eval::error::RuntimeError;

impl Runtime {
    pub(crate) fn resolve_if_reference(&self, value: Value) -> Value {
        match value {
            Value::Reference(path, scope) => self.resolve_reference(&path, scope).unwrap_or(Value::None),
            _ => value,
        }
    }

    pub(crate) fn resolve_reference(&self, path: &[String], scope: Scope) -> Result<Value, RuntimeError> {
        // 这个方法的实现应该与之前讨论的类似
        let var_name = path.last().ok_or(RuntimeError::InvalidReference)?;
        let mut context = self.context.lock().unwrap();

        match context.get_mut(var_name, scope.clone()) {
            Some(variable) => {
                // 如果变量存在，我们返回它的值
                // 注意：这里我们克隆值，以避免借用检查器的问题
                Ok(variable.value.clone())
            },
            None => {
                // 如果变量不存在，返回一个 UndefinedVariable 错误
                Err(RuntimeError::UndefinedVariable(var_name.clone()))
            }
        }
    }

    fn binary_operation<F>(&self, left: Value, right: Value, op: F) -> Value
        where
            F: Fn(f64, f64) -> f64,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Value::Number(op(a as f64, b as f64) as i64),
            // (Value::String(a), Value::String(b)) if matches!(op, |a, b| a + b) => {
            //     Value::String(a + &b)
            // }
            // 可以添加更多类型的组合
            _ => Value::None, // 或者返回一个类型错误
        }
    }
}