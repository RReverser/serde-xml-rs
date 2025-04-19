use super::{
    child::ChildDeserializer,
    plain::PlainTextDeserializer,
    reader::{Attribute, ChildReader, Element, Event, Reader},
};
use crate::{
    config::{CONTENT, TEXT},
    error::{Error, Result},
};
use log::trace;
use serde::de::IntoDeserializer;
use std::{io::Read, iter::Peekable};

pub struct MapAccess<'a, R: Read> {
    reader: ChildReader<'a, R>,
    attributes: Peekable<std::vec::IntoIter<Attribute>>,
    capture_content: bool,
}

impl<'a, R: Read> MapAccess<'a, R> {
    pub fn new_map(reader: ChildReader<'a, R>) -> Self {
        Self {
            reader,
            attributes: vec![].into_iter().peekable(),
            capture_content: false,
        }
    }
    pub fn new_struct(
        reader: ChildReader<'a, R>,
        attributes: Vec<Attribute>,
        fields: &'static [&'static str],
    ) -> Self {
        Self {
            reader,
            attributes: attributes.into_iter().peekable(),
            capture_content: fields.contains(&CONTENT),
        }
    }
}

impl<'de, R: Read> serde::de::MapAccess<'de> for MapAccess<'_, R> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        trace!("next map/struct key");
        if let Some(attr) = self.attributes.peek() {
            trace!("attribute {}", attr.qname());
            seed.deserialize(format!("@{}", attr.qname()).into_deserializer())
                .map(Some)
        } else if self.capture_content {
            trace!("#content");
            seed.deserialize(CONTENT.into_deserializer()).map(Some)
        } else {
            match self.reader.peek()? {
                Event::StartElement(element) => {
                    trace!("element '{}'", element.name);
                    seed.deserialize(element.qname().as_str().into_deserializer())
                        .map(Some)
                }
                Event::Text(_) => {
                    trace!("{}", TEXT);
                    seed.deserialize(TEXT.into_deserializer()).map(Some)
                }
                _ => Ok(None),
            }
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        if let Some(attr) = self.attributes.next() {
            seed.deserialize(PlainTextDeserializer::new(&attr.value))
        } else if self.capture_content {
            self.capture_content = false;
            seed.deserialize(ChildDeserializer::new(self.reader.child()))
        } else {
            match self.reader.peek()? {
                Event::StartElement(Element { name, .. }) => {
                    let name = name.to_string();
                    seed.deserialize(ChildDeserializer::new_with_element_name(
                        self.reader.child(),
                        name.to_string(),
                    ))
                }
                Event::Text(_) => {
                    seed.deserialize(PlainTextDeserializer::new(&self.reader.chars()?))
                }
                event => Err(Error::Unexpected {
                    expected: "start of element or text",
                    but_got: event.to_string(),
                }),
            }
        }
    }
}
