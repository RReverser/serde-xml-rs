#![doc = include_str!("lib.md")]

pub mod config;
pub mod de;
mod error;
pub mod ser;
#[cfg(test)]
mod test;

pub use crate::config::SerdeXml;
pub use crate::de::{from_reader, from_reader_with, from_str, from_str_with, Deserializer};
pub use crate::error::Error;
pub use crate::ser::{to_string, to_string_with, to_writer, to_writer_with, Serializer};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;
