//!
//!
//! # Examples
//!
//! ```rust
//! extern crate serde;
//! extern crate serde_xml_rs;
//!
//! #[macro_use]
//! extern crate serde_derive;
//!
//! use serde_xml_rs::{from_str, to_string};
//!
//! #[derive(Debug, Serialize, Deserialize, PartialEq)]
//! struct Item {
//!     name: String,
//!     source: String,
//! }
//!
//! fn main() {
//!     let src = r#"<?xml version="1.0" encoding="UTF-8"?><Item><name>Banana</name><source>Store</source></Item>"#;
//!     let should_be = Item {
//!         name: "Banana".to_string(),
//!         source: "Store".to_string(),
//!     };
//!
//!     let item: Item = from_str(src).unwrap();
//!     assert_eq!(item, should_be);
//!
//!     let reserialized_item = to_string(&item).unwrap();
//!     assert_eq!(src, reserialized_item);
//! }
//! ```


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
pub use ser::{to_string, to_writer, Serializer};
pub use de::{from_reader, from_str, Deserializer};

use serde::ser::SerializeMap;

/// Helper function for serializing lists of primitives as <name>item<name>
pub fn wrap_primitives<T: serde::ser::Serialize, S: serde::ser::Serializer>(
    items: &Vec<T>,
    serializer: S,
    name: &'static str,
) -> Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(None)?;
    for ref item in items {
        map.serialize_key(name)?;
        map.serialize_value(item)?;
    }
    map.end()
}
