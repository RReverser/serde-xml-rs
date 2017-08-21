#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
extern crate xml;

#[cfg(test)]
#[macro_use]
extern crate serde_derive;

#[macro_use]
mod error;
pub mod de;
pub mod ser;

pub use error::{Error, ErrorKind};
pub use xml::reader::{EventReader, ParserConfig};
pub use ser::{serialize, to_string, to_writer, Serializer};
pub use de::{deserialize, from_reader, from_str, Deserializer};
