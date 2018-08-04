use failure::{Context, Fail, Backtrace};

use std::fmt;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone, Fail)]
pub enum ErrorKind {
    #[fail(display = "Failed to successfully access Photos endpoint.")]
    Photos
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(inner: ErrorKind) -> Self {
        Error {inner: Context::new(inner)}
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Error {inner}
    }
}