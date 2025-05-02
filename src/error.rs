#[derive(Debug)]
pub enum IroncladError {
    MyBad,
}

impl std::fmt::Display for IroncladError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IroncladError::MyBad => write!(f, "Something's wrong"),
        }
    }
}

impl std::error::Error for IroncladError {}

pub type IroncladResult<T> = Result<T, IroncladError>;
