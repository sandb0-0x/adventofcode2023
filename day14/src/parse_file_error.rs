use std::{error, fmt};

#[derive(Debug)]
pub enum ParseFileError {
    Default,
    CustomError(String),
    WrappedError(Box<dyn error::Error>),
}

impl ParseFileError {
    pub fn from_str(s: String) -> Self {
        Self::CustomError(s)
    }

    pub fn from_err<E: error::Error + 'static>(err: E) -> Self {
        Self::WrappedError(Box::new(err))
    }
}

impl fmt::Display for ParseFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseFileError::Default => write!(f, "Could not parse file contents"),
            ParseFileError::CustomError(err_str) => {
                write!(f, "Could not parse file contents: {err_str}")
            }
            ParseFileError::WrappedError(err) => write!(f, "Could not parse file contents: {err}"),
        }
    }
}

impl error::Error for ParseFileError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseFileError::WrappedError(err) => Some(err.as_ref()),
            _ => None,
        }
    }
}
