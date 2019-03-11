use std::io::Read;

use serde::de;
use xml::reader::{EventReader, ParserConfig, XmlEvent};
use xml::name::OwnedName;

use error::{Error, ErrorKind, Result};
use self::map::MapAccess;
use self::seq::SeqAccess;
use self::var::EnumAccess;

mod map;
mod seq;
mod var;

/// A convenience method for deserialize some object from a string.
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde;
/// # extern crate serde_xml_rs;
/// # use serde_xml_rs::from_str;
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct Item {
///     name: String,
///     source: String,
/// }
/// # fn main() {
/// let s = r##"<item name="hello" source="world.rs" />"##;
/// let item: Item = from_str(s).unwrap();
/// assert_eq!(item, Item { name: "hello".to_string(),source: "world.rs".to_string()});
/// # }
/// ```
pub fn from_str<'de, T: de::Deserialize<'de>>(s: &str) -> Result<T> {
    from_reader(s.as_bytes())
}


/// A convenience method for deserialize some object from a reader.
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde;
/// # extern crate serde_xml_rs;
/// # use serde_xml_rs::from_reader;
/// #[derive(Debug, Deserialize, PartialEq)]
/// struct Item {
///     name: String,
///     source: String,
/// }
/// # fn main() {
/// let s = r##"<item name="hello" source="world.rs" />"##;
/// let item: Item = from_reader(s.as_bytes()).unwrap();
/// assert_eq!(item, Item { name: "hello".to_string(),source: "world.rs".to_string()});
/// # }
/// ```
pub fn from_reader<'de, R: Read, T: de::Deserialize<'de>>(reader: R) -> Result<T> {
    T::deserialize(&mut Deserializer::new_from_reader(reader))
}

pub struct Deserializer<R: Read> {
    depth: usize,
    reader: EventReader<R>,
    peeked: Option<XmlEvent>,
    is_map_value: bool,
}

impl<'de, R: Read> Deserializer<R> {
    pub fn new(reader: EventReader<R>) -> Self {
        Deserializer {
            depth: 0,
            reader: reader,
            peeked: None,
            is_map_value: false,
        }
    }

    pub fn new_from_reader(reader: R) -> Self {
        let config = ParserConfig::new()
            .trim_whitespace(true)
            .whitespace_to_characters(true)
            .cdata_to_characters(true)
            .ignore_comments(true)
            .coalesce_characters(true);

        Self::new(EventReader::new_with_config(reader, config))
    }

    fn peek(&mut self) -> Result<&XmlEvent> {
        if self.peeked.is_none() {
            self.peeked = Some(self.inner_next()?);
        }
        debug_expect!(self.peeked.as_ref(), Some(peeked) => {
            debug!("Peeked {:?}", peeked);
            Ok(peeked)
        })
    }

    fn inner_next(&mut self) -> Result<XmlEvent> {
        loop {
            match self.reader.next().map_err(ErrorKind::Syntax)? {
                XmlEvent::StartDocument { .. } |
                XmlEvent::ProcessingInstruction { .. } |
                XmlEvent::Comment(_) => { /* skip */ },
                other => return Ok(other),
            }
        }
    }

    fn next(&mut self) -> Result<XmlEvent> {
        let next = if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.inner_next()?
        };
        match next {
            XmlEvent::StartElement { .. } => {
                self.depth += 1;
            },
            XmlEvent::EndElement { .. } => {
                self.depth -= 1;
            },
            _ => {},
        }
        debug!("Fetched {:?}", next);
        Ok(next)
    }

    fn set_map_value(&mut self) {
        self.is_map_value = true;
    }

    pub fn unset_map_value(&mut self) -> bool {
        ::std::mem::replace(&mut self.is_map_value, false)
    }

    fn read_inner_value<V: de::Visitor<'de>, T, F: FnOnce(&mut Self) -> Result<T>>(
        &mut self,
        f: F,
    ) -> Result<T> {
        if self.unset_map_value() {
            debug_expect!(self.next(), Ok(XmlEvent::StartElement { name, .. }) => {
                let result = f(self)?;
                self.expect_end_element(name)?;
                Ok(result)
            })
        } else {
            f(self)
        }
    }

    fn expect_end_element(&mut self, start_name: OwnedName) -> Result<()> {
        expect!(self.next()?, XmlEvent::EndElement { name, .. } => {
            if name == start_name {
                Ok(())
            } else {
                Err(ErrorKind::Custom(format!(
                    "End tag </{}> didn't match the start tag <{}>",
                    name.local_name,
                    start_name.local_name
                )).into())
            }
        })
    }

    fn prepare_parse_type<V: de::Visitor<'de>>(&mut self) -> Result<String> {
        if let XmlEvent::StartElement { .. } = *self.peek()? {
            self.set_map_value()
        }
        self.read_inner_value::<V, String, _>(|this| {
            if let XmlEvent::EndElement { .. } = *this.peek()? {
                return Err(
                    ErrorKind::UnexpectedToken("EndElement".into(), "Characters".into()).into(),
                );
            }

            expect!(this.next()?, XmlEvent::Characters(s) => {
                return Ok(s)
            })
        })
    }
}

macro_rules! deserialize_type {
    ($deserialize:ident => $visit:ident) => {
        fn $deserialize<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
            let value = self.prepare_parse_type::<V>()?.parse()?;
            visitor.$visit(value)
        }
    }
}

impl<'de, 'a, R: Read> de::Deserializer<'de> for &'a mut Deserializer<R> {
    type Error = Error;

    forward_to_deserialize_any! {
        newtype_struct identifier
    }

    fn deserialize_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.unset_map_value();
        expect!(self.next()?, XmlEvent::StartElement { name, attributes, .. } => {
            let map_value = visitor.visit_map(MapAccess::new(
                self,
                attributes,
                fields.contains(&"$value")
            ))?;
            self.expect_end_element(name)?;
            Ok(map_value)
        })
    }

    deserialize_type!(deserialize_i8 => visit_i8);
    deserialize_type!(deserialize_i16 => visit_i16);
    deserialize_type!(deserialize_i32 => visit_i32);
    deserialize_type!(deserialize_i64 => visit_i64);
    deserialize_type!(deserialize_u8 => visit_u8);
    deserialize_type!(deserialize_u16 => visit_u16);
    deserialize_type!(deserialize_u32 => visit_u32);
    deserialize_type!(deserialize_u64 => visit_u64);
    deserialize_type!(deserialize_f32 => visit_f32);
    deserialize_type!(deserialize_f64 => visit_f64);

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if let XmlEvent::StartElement { .. } = *self.peek()? {
            self.set_map_value()
        }
        self.read_inner_value::<V, V::Value, _>(|this| {
            if let XmlEvent::EndElement { .. } = *this.peek()? {
                return visitor.visit_bool(false);
            }
            expect!(this.next()?, XmlEvent::Characters(s) => {
                match s.as_str() {
                    "true" | "1" => visitor.visit_bool(true),
                    "false" | "0" => visitor.visit_bool(false),
                    _ => Err(ErrorKind::UnexpectedToken(
                        "boolean [true,false,1,0]".into(),
                        format!("{}", s)
                    ).into()),
                }
                
            })
        })
    }

    fn deserialize_char<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_str<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_bytes<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_byte_buf<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_unit<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if let XmlEvent::StartElement { .. } = *self.peek()? {
            self.set_map_value()
        }
        self.read_inner_value::<V, V::Value, _>(
            |this| expect!(this.peek()?, &XmlEvent::EndElement { .. } => visitor.visit_unit()),
        )
    }

    fn deserialize_unit_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_tuple_struct<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_tuple<V: de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(SeqAccess::new(self, Some(len)))
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.read_inner_value::<V, V::Value, _>(|this| visitor.visit_enum(EnumAccess::new(this)))
    }

    fn deserialize_string<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        if let XmlEvent::StartElement { .. } = *self.peek()? {
            self.set_map_value()
        }
        self.read_inner_value::<V, V::Value, _>(|this| {
            if let XmlEvent::EndElement { .. } = *this.peek()? {
                return visitor.visit_str("");
            }
            expect!(this.next()?, XmlEvent::Characters(s) => {
                visitor.visit_string(s)
            })
        })
    }

    fn deserialize_seq<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_seq(SeqAccess::new(self, None))
    }

    fn deserialize_map<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.unset_map_value();
        expect!(self.next()?, XmlEvent::StartElement { name, attributes, .. } => {
            let map_value = visitor.visit_map(MapAccess::new(self, attributes, false))?;
            self.expect_end_element(name)?;
            Ok(map_value)
        })
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match *self.peek()? {
            XmlEvent::EndElement { .. } => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_ignored_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.unset_map_value();
        let depth = self.depth;
        loop {
            self.next()?;
            if self.depth == depth {
                break;
            }
        }
        visitor.visit_unit()
    }

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match *self.peek()? {
            XmlEvent::StartElement { .. } => self.deserialize_map(visitor),
            XmlEvent::EndElement { .. } => self.deserialize_unit(visitor),
            _ => self.deserialize_string(visitor),
        }
    }
}
