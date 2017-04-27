use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use serde::de::{self, IntoDeserializer};

use {Deserializer};
use error::{Result, Error};

pub struct MapAccess<'a, R: 'a + Read> {
    attrs: ::std::vec::IntoIter<OwnedAttribute>,
    next_value: Option<String>,
    de: &'a mut Deserializer<R>,
    inner_value: bool
}

impl<'a, R: 'a + Read> MapAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, attrs: Vec<OwnedAttribute>, inner_value: bool) -> Self {
        MapAccess {
            attrs: attrs.into_iter(),
            next_value: None,
            de: de,
            inner_value: inner_value
        }
    }
}

impl<'de, 'a, R: 'a + Read> de::MapAccess<'de> for MapAccess<'a, R> {
    type Error = Error;

    fn next_key_seed<K: de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>> {
        debug_assert_eq!(self.next_value, None);
        match self.attrs.next() {
            Some(OwnedAttribute { name, value }) => {
                self.next_value = Some(value);
                seed.deserialize(name.local_name.into_deserializer()).map(Some)
            }
            None => {
                match *self.de.peek()? {
                    XmlEvent::StartElement { ref name, .. } => {
                        seed.deserialize(if !self.inner_value {
                            name.local_name.as_str()
                        } else {
                            "$value"
                        }.into_deserializer()).map(Some)
                    }
                    XmlEvent::Characters(_) => {
                        seed.deserialize("$value".into_deserializer()).map(Some)
                    }
                    _ => Ok(None)
                }
            }
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        match self.next_value.take() {
            Some(value) => seed.deserialize(AttrValueDeserializer(value)),
            None => {
                if !self.inner_value {
                    if let &XmlEvent::StartElement { .. } = self.de.peek()? {
                        self.de.set_map_value();
                    }
                }
                let result = seed.deserialize(&mut *self.de)?;
                Ok(result)
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.attrs.size_hint().1
    }
}

struct AttrValueDeserializer(String);

impl<'de> de::Deserializer<'de> for AttrValueDeserializer {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.0)
    }

    fn deserialize_u8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_u8(u)
    }

    fn deserialize_u16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_u16(u)
    }

    fn deserialize_u32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_u32(u)
    }

    fn deserialize_u64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_u64(u)
    }

    fn deserialize_i8<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_i8(u)
    }

    fn deserialize_i16<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_i16(u)
    }

    fn deserialize_i32<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_i32(u)
    }

    fn deserialize_i64<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        let u = self.0.parse().map_err(|err| Error::ParseIntError(err))?;
        visitor.visit_i64(u)
    }

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V
    ) -> Result<V::Value> {
        visitor.visit_enum(self.0.into_deserializer())
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(!self.0.is_empty())
    }

    forward_to_deserialize_any! {
        f32 f64 char str string unit
        seq bytes map unit_struct newtype_struct tuple_struct
        struct identifier tuple ignored_any byte_buf
    }
}
