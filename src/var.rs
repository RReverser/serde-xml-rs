use std::io::Read;
use xml::name::OwnedName;
use xml::reader::XmlEvent;
use {Deserializer, Error};
use serde::de::{self, DeserializeSeed, Deserializer as SerdeDeserializer, Visitor, Error as SerdeError};
use serde::de::value::ValueDeserializer;
use VResult;

pub struct EnumVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>
}

impl<'a, R: 'a + Read> EnumVisitor<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        EnumVisitor {
            de: de
        }
    }
}

impl<'a, R: 'a + Read> de::EnumVisitor for EnumVisitor<'a, R> {
    type Error = Error;
    type Variant = VariantVisitor<'a, R>;

    fn visit_variant_seed<V: DeserializeSeed>(self, seed: V) -> Result<(V::Value, VariantVisitor<'a, R>), Error> {
        let name = expect!(self.de.peek()?, &XmlEvent::Characters(ref name) | &XmlEvent::StartElement { name: OwnedName { local_name: ref name, .. }, .. } => {
            seed.deserialize(name.as_str().into_deserializer())
        })?;
        Ok((name, VariantVisitor::new(self.de)))
    }
}

pub struct VariantVisitor<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>
}

impl<'a, R: 'a + Read> VariantVisitor<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>) -> Self {
        VariantVisitor {
            de: de
        }
    }
}

impl<'a, R: 'a + Read> de::VariantVisitor for VariantVisitor<'a, R> {
    type Error = Error;

    fn visit_unit(self) -> Result<(), Error> {
        match self.de.next()? {
            XmlEvent::StartElement { attributes, .. } => {
                if attributes.len() == 0 {
                    self.de.expect_end_element()
                } else {
                    Err(Error::invalid_length(attributes.len(), &"0"))
                }
            },
            _ => Ok(())
        }
    }

    fn visit_newtype_seed<T: DeserializeSeed>(self, seed: T) -> Result<T::Value, Error> {
        self.de.set_map_value();
        seed.deserialize(&mut *self.de)
    }

    fn visit_tuple<V: Visitor>(self, len: usize, visitor: V) -> VResult<V> {
        self.de.set_map_value();
        self.de.deserialize_tuple(len, visitor)
    }

    fn visit_struct<V: Visitor>(self, _fields: &'static [&'static str], visitor: V) -> VResult<V> {
        self.de.set_map_value();
        self.de.deserialize_map(visitor)
    }
}