use std::io::Read;
use xml::attribute::OwnedAttribute;
use xml::reader::XmlEvent;
use {Deserializer, Error, VResult};
use serde::de::{self, DeserializeSeed, Visitor};
use serde::de::value::ValueDeserializer;

pub struct MapVisitor<'a, R: 'a + Read> {
    attrs: ::std::vec::IntoIter<OwnedAttribute>,
    next_value: Option<String>,
    de: &'a mut Deserializer<R>,
    inner_value: bool
}

impl<'a, R: 'a + Read> MapVisitor<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, attrs: Vec<OwnedAttribute>, inner_value: bool) -> Self {
        MapVisitor {
            attrs: attrs.into_iter(),
            next_value: None,
            de: de,
            inner_value: inner_value
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

    fn visit_value_seed<V: DeserializeSeed>(&mut self, seed: V) -> Result<V::Value, Error> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.attrs.size_hint().0, None)
    }
}

struct AttrValueDeserializer(String);

impl de::Deserializer for AttrValueDeserializer {
    type Error = Error;

    fn deserialize<V: Visitor>(self, visitor: V) -> VResult<V> {
        visitor.visit_string(self.0)
    }

    fn deserialize_enum<V: Visitor>(self, _name: &str, _variants: &'static [&'static str], visitor: V) -> VResult<V> {
        visitor.visit_enum(self.0.into_deserializer())
    }

    fn deserialize_option<V: Visitor>(self, visitor: V) -> VResult<V> {
        visitor.visit_some(self)
    }

    fn deserialize_bool<V: Visitor>(self, visitor: V) -> VResult<V> {
        visitor.visit_bool(self.0.is_empty())
    }

    forward_to_deserialize! {
        u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit
        seq seq_fixed_size bytes map unit_struct newtype_struct tuple_struct
        struct struct_field tuple ignored_any byte_buf
    }
}