use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    TypeError(String),
    UndefinedVariable(String),
    IndexOutOfBounds(usize, usize),
    DivisionByZero,
    InvalidReference,
    InvalidOperation(String),
    StackOverflow,
    MemoryLimit,
    FileNotFound(String),
    IOError(String),
    Custom(String),
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::TypeError(msg) => write!(f, "Type Error: {}", msg),
            RuntimeError::UndefinedVariable(name) => write!(f, "Undefined Variable: {}", name),
            RuntimeError::IndexOutOfBounds(index, len) =>
                write!(f, "Index Out of Bounds: index {} is out of bounds for length {}", index, len),
            RuntimeError::DivisionByZero => write!(f, "Division by Zero"),
            RuntimeError::InvalidReference => write!(f, "Invalid Reference"),
            RuntimeError::InvalidOperation(msg) => write!(f, "Invalid Operation: {}", msg),
            RuntimeError::StackOverflow => write!(f, "Stack Overflow"),
            RuntimeError::MemoryLimit => write!(f, "Memory Limit Exceeded"),
            RuntimeError::FileNotFound(path) => write!(f, "File Not Found: {}", path),
            RuntimeError::IOError(msg) => write!(f, "IO Error: {}", msg),
            RuntimeError::Custom(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl std::error::Error for RuntimeError {}