use std::{error, fmt, result};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    Internal,
    InvalidData,
    ApiError(String),
}

impl From<reqwest::Error> for ErrorKind {
    fn from(_: reqwest::Error) -> Self {
        ErrorKind::Internal
    }
}

#[derive(Debug)]
pub struct Error {
    msg: String,
    code: &'static str,
    kind: ErrorKind,
}

impl Error {
    pub fn new(msg: impl Into<String>, kind: impl Into<ErrorKind>) -> Error {
        Error {
            msg: msg.into(),
            code: "NA",
            kind: kind.into(),
        }
    }

    pub fn from_builder(target: &'static str, missing: &'static str) -> Error {
        Error {
            msg: format!("{} cannot be constructed without {}", target, missing),
            code: "NA",
            kind: ErrorKind::Internal,
        }
    }

    pub fn with_code(
        msg: impl Into<String>,
        code: &'static str,
        kind: impl Into<ErrorKind>,
    ) -> Error {
        Error {
            msg: msg.into(),
            code,
            kind: kind.into(),
        }
    }

    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.msg)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Internal => None,
            ErrorKind::InvalidData => None,
            ErrorKind::ApiError(_) => None,
        }
    }
}

pub trait Context<T> {
    fn context(self, msg: impl Into<String>) -> Result<T>;

    fn with_context<F, S>(self, cb: F) -> Result<T>
    where
        F: Fn() -> S,
        S: Into<String>;
}

impl<T, E: Into<ErrorKind>> Context<T> for result::Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|err| Error::new(msg, err))
    }

    fn with_context<F, S>(self, cb: F) -> Result<T>
    where
        F: Fn() -> S,
        S: Into<String>,
    {
        self.map_err(move |err| Error::new(cb(), err))
    }
}
