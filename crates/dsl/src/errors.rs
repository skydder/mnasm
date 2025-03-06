#[derive(Debug)]
pub enum DSLError {
    Tokenize(String),
    Parse(String),
    Eval(String),
}

impl std::fmt::Display for DSLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DSLError::Tokenize(msg) => write!(f, "error occured while tokenizing: {}", msg),
            DSLError::Parse(msg) => write!(f, "error occured while parsing: {}", msg),
            DSLError::Eval(msg) => write!(f, "error occured while evaluating: {}", msg),
        }
    }
}
impl std::error::Error for DSLError {}

pub type DSLResult<T> = Result<T, DSLError>;
