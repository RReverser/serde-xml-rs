use std::fmt::Display;
use serde::de::Error as DeError;
use serde::ser::Error as SerError;

error_chain! {
    types {
        Error,
        ErrorKind,
        ResultExt,
        Result;
    }

    foreign_links {
        Io(::std::io::Error);
        FromUtf8Error(::std::string::FromUtf8Error);
        ParseIntError(::std::num::ParseIntError);
        ParseFloatError(::std::num::ParseFloatError);
        ParseBoolError(::std::str::ParseBoolError);
        Syntax(::xml::reader::Error);
    }

    errors {
        UnexpectedToken(token: String, found: String) {
            description("unexpected token")
            display("Expected token {}, found {}", token, found)
        }
        Custom(field: String) {
            description("other error")
            display("custom: '{}'", field)
        }
        UnsupportedOperation(operation: String) {
            description("unsupported operation")
            display("unsupported operation: '{}'", operation)
        }
        NonPrimitiveKey {
            description("Map key has non-primitive value")
            display("Map key has non-primitive value")
        }
    }
}

macro_rules! expect {
    ($actual: expr, $($expected: pat)|+ => $if_ok: expr) => {
        match $actual {
            $($expected)|+ => $if_ok,
            actual => Err($crate::ErrorKind::UnexpectedToken(
                stringify!($($expected)|+).to_string(), format!("{:?}",actual)
            ).into()) as Result<_>
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

impl DeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}

impl SerError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}
