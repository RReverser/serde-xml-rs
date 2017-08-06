use xml::reader;
use std::fmt::{self, Display, Debug};
use std::error::Error as StdError;
use std::num::ParseIntError;
use std::io;
use serde::de::Error as DeError;
use serde::ser::Error as SerError;

pub enum Error {
    ParseIntError(ParseIntError),
    Syntax(reader::Error),
    Custom(String),
    Io(io::Error),
    UnsupportedOperation(String),
}

pub type VResult<V> = Result<V, Error>;

macro_rules! expect {
    ($actual: expr, $($expected: pat)|+ => $if_ok: expr) => {
        match $actual {
            $($expected)|+ => $if_ok,
            actual => Err($crate::Error::Custom(format!(
                "Expected token {}, found {:?}",
                stringify!($($expected)|+),
                actual
            )))
        }
    }
}

#[cfg(debug_assertions)]
macro_rules! debug_expect {
    ($actual: expr, $($expected: pat)|+ => $if_ok: expr) => {
        match $actual {
            $($expected)|+ => $if_ok,
            actual => panic!(
                "Internal error: Expected token {}, found {:?}",
                stringify!($($expected)|+),
                actual
            )
        }
    }
}

#[cfg(not(debug_assertions))]
macro_rules! debug_expect {
    ($actual: expr, $($expected: pat)|+ => $if_ok: expr) => {
        match $actual {
            $($expected)|+ => $if_ok,
            _ => unreachable!()
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseIntError(ref error) => Display::fmt(error, fmt),
            Error::Syntax(ref error) => Display::fmt(error, fmt),
            Error::Custom(ref display) => Display::fmt(display, fmt),
            Error::Io(ref err) => Display::fmt(err, fmt),
            Error::UnsupportedOperation(ref msg) => write!(fmt, "Unsupported Operation: {}", msg),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseIntError(ref error) => Display::fmt(error, fmt),
            Error::Syntax(ref error) => Debug::fmt(error, fmt),
            Error::Custom(ref display) => Display::fmt(display, fmt),
            Error::Io(ref err) => Display::fmt(err, fmt),
            Error::UnsupportedOperation(ref msg) => write!(fmt, "Unsupported Operation: {}", msg),
        }
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseIntError(ref error) => error.description(),
            Error::Syntax(ref error) => error.description(),
            Error::Io(ref error) => error.description(),
            Error::Custom(_) => "other error",
            Error::UnsupportedOperation(_) => "Unsupported Operation",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::ParseIntError(ref error) => Some(error),
            Error::Syntax(ref error) => Some(error),
            Error::Io(ref error) => Some(error),
            _ => None,
        }
    }
}

impl DeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}
