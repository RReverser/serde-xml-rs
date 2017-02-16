extern crate xml;
#[macro_use] extern crate serde;

#[macro_use] mod error;
mod map;
mod seq;
mod var;

pub use error::Error;
pub use xml::reader::{EventReader, ParserConfig};

use error::VResult;
use xml::reader::XmlEvent;
use serde::de::{self, Visitor};
use std::io::Read;
use map::MapVisitor;
use seq::SeqVisitor;
use var::EnumVisitor;

pub struct Deserializer<R: Read> {
    depth: usize,
    reader: EventReader<R>,
    peeked: Option<XmlEvent>,
    is_map_value: bool
}

impl<R: Read> Deserializer<R> {
    pub fn new(reader: EventReader<R>) -> Self {
        Deserializer {
            depth: 0,
            reader: reader,
            peeked: None,
            is_map_value: false
        }
    }

    fn peek(&mut self) -> Result<&XmlEvent, Error> {
        if self.peeked.is_none() {
            self.peeked = Some(self.inner_next()?);
        }
        debug_expect!(self.peeked.as_ref(), Some(peeked) => Ok(peeked))
    }

    fn inner_next(&mut self) -> Result<XmlEvent, Error> {
        loop {
            match self.reader.next().map_err(Error::Syntax)? {
                XmlEvent::StartDocument { .. } |
                XmlEvent::EndDocument { .. } |
                XmlEvent::ProcessingInstruction { .. } |
                XmlEvent::Comment(_) => {/* skip */}

                other => return Ok(other)
            }
        }
    }

    fn next(&mut self) -> Result<XmlEvent, Error> {
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
            _ => {}
        }
        Ok(next)
    }

    fn set_map_value(&mut self) {
        self.is_map_value = true;
    }

    pub fn unset_map_value(&mut self) -> bool {
        ::std::mem::replace(&mut self.is_map_value, false)
    }

    fn skip_map_value_header(&mut self) -> Result<(), ()> {
        if self.unset_map_value() {
            debug_expect!(self.next(), Ok(XmlEvent::StartElement { .. }) => Ok(()))
        } else {
            Err(())
        }
    }

    fn expect_end_element(&mut self) -> Result<(), Error> {
        expect!(self.next()?, XmlEvent::EndElement { .. } => Ok(()))
    }
}

impl<'a, R: Read> de::Deserializer for &'a mut Deserializer<R> {
    type Error = Error;

    forward_to_deserialize! {
        bool u8 u16 u32 u64 i8 i16 i64 f32 f64 char str unit
        bytes byte_buf unit_struct newtype_struct struct
        tuple_struct struct_field tuple
    }

    fn deserialize_i32<V: Visitor>(self, visitor: V) -> VResult<V> {
        self.deserialize_string(visitor)
    }

    fn deserialize_enum<V: Visitor>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> VResult<V> {
        let value_header = self.skip_map_value_header();
        let result = visitor.visit_enum(EnumVisitor::new(self))?;
        if value_header.is_ok() {
            self.expect_end_element()?;
        }
        Ok(result)
    }

    fn deserialize_string<V: Visitor>(self, visitor: V) -> VResult<V> {
        let value_header = self.skip_map_value_header();
        expect!(self.next()?, XmlEvent::Characters(s) => {
            let result = visitor.visit_string(s)?;
            if value_header.is_ok() {
                self.expect_end_element()?;
            }
            Ok(result)
        })
    }

    fn deserialize_seq<V: Visitor>(self, visitor: V) -> VResult<V> {
        visitor.visit_seq(SeqVisitor::new(self, None))
    }

    fn deserialize_seq_fixed_size<V: Visitor>(self, len: usize, visitor: V) -> VResult<V> {
        visitor.visit_seq(SeqVisitor::new(self, Some(len)))
    }

    fn deserialize_map<V: Visitor>(self, visitor: V) -> VResult<V> {
        self.unset_map_value();
        expect!(self.next()?, XmlEvent::StartElement { attributes, .. } => {
            let map_value = visitor.visit_map(MapVisitor::new(self, attributes))?;
            self.expect_end_element()?;
            Ok(map_value)
        })
    }

    fn deserialize_option<V: Visitor>(self, visitor: V) -> VResult<V> {
        if let Ok(_) = self.skip_map_value_header() {
            let result = visitor.visit_some(&mut *self)?;
            self.expect_end_element()?;
            return Ok(result);
        }
        match *self.peek()? {
            XmlEvent::EndElement { .. } => visitor.visit_none(),
            _ => visitor.visit_some(self)
        }
    }

    fn deserialize_ignored_any<V: Visitor>(self, visitor: V) -> VResult<V> {
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

    fn deserialize<V: Visitor>(self, visitor: V) -> VResult<V> {
        match *self.peek()? {
            XmlEvent::StartElement { .. } => {
                self.deserialize_map(visitor)
            }
            XmlEvent::EndElement { .. } => {
                self.unset_map_value();
                visitor.visit_unit()
            }
            _ => {
                self.deserialize_string(visitor)
            }
        }
    }
}
