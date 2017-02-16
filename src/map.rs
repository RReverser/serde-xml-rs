use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use {Deserializer, Error};
use serde::de::{self, DeserializeSeed};
use serde::de::value::ValueDeserializer;

pub struct MapVisitor<'a, R: 'a + Read> {
    attrs: ::std::vec::IntoIter<OwnedAttribute>,
    next_value: Option<String>,
    de: &'a mut Deserializer<R>
}

impl<'a, R: 'a + Read> MapVisitor<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, attrs: Vec<OwnedAttribute>) -> Self {
        MapVisitor {
            attrs: attrs.into_iter(),
            next_value: None,
            de: de
        }
    }
}

impl<'a, R: 'a + Read> de::MapVisitor for MapVisitor<'a, R> {
    type Error = Error;

    fn visit_key_seed<K: DeserializeSeed>(&mut self, seed: K) -> Result<Option<K::Value>, Error> {
        debug_assert_eq!(self.next_value, None);
        match self.attrs.next() {
            Some(OwnedAttribute { name, value }) => {
                self.next_value = Some(value);
                seed.deserialize(name.local_name.into_deserializer()).map(Some)
            }
            None => {
                match *self.de.peek()? {
                    XmlEvent::StartElement { ref name, .. } => {
                        seed.deserialize(name.local_name.as_str().into_deserializer()).map(Some)
                    }
                    XmlEvent::Characters(_) => {
                        seed.deserialize("$value".into_deserializer()).map(Some)
                    }
                    _ => Ok(None)
                }
            }
        }
    }

    fn visit_value_seed<V: DeserializeSeed>(&mut self, seed: V) -> Result<V::Value, Error> {
        match self.next_value.take() {
            Some(value) => seed.deserialize(value.into_deserializer()),
            None => {
                let is_char = match *self.de.peek()? {
                    XmlEvent::StartElement { .. } => {
                        self.de.set_map_value();
                        false
                    },
                    XmlEvent::Characters(_) => {
                        true
                    }
                    _ => unreachable!()
                };
                let result = seed.deserialize(&mut *self.de)?;
                if is_char {
                    expect!(self.de.peek()?, &XmlEvent::EndElement { .. } => Ok(()))?;
                }
                Ok(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.attrs.size_hint().0, None)
    }
}
