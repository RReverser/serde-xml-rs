use std::io::Read;

use serde::de::{self, IntoDeserializer, Unexpected};
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;

use Deserializer;
use error::{Error, Result};

pub struct MapAccess<'a, R: 'a + Read> {
    attrs: ::std::vec::IntoIter<OwnedAttribute>,
    next_value: Option<String>,
    de: &'a mut Deserializer<R>,
    inner_value: bool,
}

impl<'a, R: 'a + Read> MapAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, attrs: Vec<OwnedAttribute>, inner_value: bool) -> Self {
        MapAccess {
            attrs: attrs.into_iter(),
            next_value: None,
            de,
            inner_value,
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
                seed.deserialize(name.local_name.into_deserializer())
                    .map(Some)
            },
            None => match *self.de.peek()? {
                XmlEvent::StartElement { ref name, .. } => seed.deserialize(
                    if !self.inner_value {
                        name.local_name.as_str()
                    } else {
                        "$value"
                    }.into_deserializer(),
                ).map(Some),
                XmlEvent::Characters(_) => seed.deserialize("$value".into_deserializer()).map(Some),
                _ => Ok(None),
            },
        }
    }

    fn next_value_seed<V: de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value> {
        match self.next_value.take() {
            Some(value) => seed.deserialize(AttrValueDeserializer(value)),
            None => {
                if !self.inner_value {
                    if let XmlEvent::StartElement { .. } = *self.de.peek()? {
                        self.de.set_map_value();
                    }
                }
                let result = seed.deserialize(&mut *self.de)?;
                Ok(result)
            },
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.attrs.size_hint().1
    }
}

struct AttrValueDeserializer(String);

macro_rules! deserialize_type_attr {
    ($deserialize:ident => $visit:ident) => {
        fn $deserialize<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
            visitor.$visit(self.0.parse()?)
        }
    }
}

impl<'de> de::Deserializer<'de> for AttrValueDeserializer {
    type Error = Error;

    fn deserialize_any<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_string(self.0)
    }

    deserialize_type_attr!(deserialize_i8 => visit_i8);
    deserialize_type_attr!(deserialize_i16 => visit_i16);
    deserialize_type_attr!(deserialize_i32 => visit_i32);
    deserialize_type_attr!(deserialize_i64 => visit_i64);
    deserialize_type_attr!(deserialize_u8 => visit_u8);
    deserialize_type_attr!(deserialize_u16 => visit_u16);
    deserialize_type_attr!(deserialize_u32 => visit_u32);
    deserialize_type_attr!(deserialize_u64 => visit_u64);
    deserialize_type_attr!(deserialize_f32 => visit_f32);
    deserialize_type_attr!(deserialize_f64 => visit_f64);

    fn deserialize_enum<V: de::Visitor<'de>>(
        self,
        _name: &str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_enum(self.0.into_deserializer())
    }

    fn deserialize_option<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_some(self)
    }

    fn deserialize_bool<V: de::Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        match self.0.as_str() {
            "true" | "1" => visitor.visit_bool(true),
            "false" | "0" => visitor.visit_bool(false),
            _ => Err(de::Error::invalid_value(Unexpected::Str(&self.0), &"a boolean")),
        }
    }

    forward_to_deserialize_any! {
        char str string unit seq bytes map unit_struct newtype_struct tuple_struct
        struct identifier tuple ignored_any byte_buf
    }
}
