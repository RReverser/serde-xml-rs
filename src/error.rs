use std::fmt::Display;

use serde::de::Error as SerdeError;

error_chain! {
    types {
        Error,
        ErrorKind,
        ResultExt,
        Result;
    }

    foreign_links {
        ParseIntError(::std::num::ParseIntError);
        ParseFloatError(::std::num::ParseFloatError);
        ParseBoolError(::std::str::ParseBoolError);
        ParseError(::std::string::ParseError);
        Syntax(::xml::reader::Error);
    }

    errors {
        Custom(field: String) {
            description("other error")
            display("custom: '{}'", field)
        }
    }
}

macro_rules! expect {
    ($actual: expr, $($expected: pat)|+ => $if_ok: expr) => {
        match $actual {
            $($expected)|+ => $if_ok,
            actual => Err($crate::ErrorKind::Custom(format!("Expected token {}, found {:?}", stringify!($($expected)|+), actual)).into())
        }
    }
}

#[cfg(debug_assertions)]
macro_rules! debug_expect {
    ($actual: expr, $($expected: pat)|+ => $if_ok: expr) => {
        match $actual {
            $($expected)|+ => $if_ok,
            actual => panic!("Internal error: Expected token {}, found {:?}", stringify!($($expected)|+), actual)
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

impl SerdeError for Error {
    fn custom<T: Display>(msg: T) -> Self {
        ErrorKind::Custom(msg.to_string()).into()
    }
}
