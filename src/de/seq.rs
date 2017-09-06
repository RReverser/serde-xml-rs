use std::io::Read;

use serde::de;
use xml::reader::XmlEvent;

use de::Deserializer;
use error::{Error, ErrorKind, Result};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    max_size: Option<usize>,
    expected_name: Option<String>,
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, max_size: Option<usize>) -> Result<Self> {
        let expected_name: Result<Option<String>> = if de.unset_map_value() {
            match de.peek() {
                Ok(&XmlEvent::StartElement { ref name, .. }) => Ok(Some(name.local_name.clone())),
                Ok(&XmlEvent::EndElement { .. }) => Ok(None),
                other => Err(
                    ErrorKind::Custom(format!(
                        "Expected StartElement or EndElement, found {:?}",
                        other
                    )).into(),
                ),
            }
        } else {
            Ok(None)
        };
        let expected_name = expected_name?;
        Ok(SeqAccess {
            de: de,
            max_size: max_size,
            expected_name: expected_name,
        })
    }
}

impl<'de, 'a, R: 'a + Read> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: de::DeserializeSeed<'de>>(
        &mut self,
        seed: T,
    ) -> Result<Option<T::Value>> {
        match self.max_size.as_mut() {
            Some(&mut 0) => {
                return Ok(None);
            },
            Some(max_size) => {
                *max_size -= 1;
            },
            None => {},
        }
        let more = match (self.de.peek()?, self.expected_name.as_ref()) {
            (&XmlEvent::StartElement { ref name, .. }, Some(expected_name)) => {
                &name.local_name == expected_name
            },
            (&XmlEvent::EndElement { .. }, None) |
            (_, Some(_)) |
            (&XmlEvent::EndDocument { .. }, _) => false,
            (_, None) => true,
        };
        if more {
            if self.expected_name.is_some() {
                self.de.set_map_value();
            }
            seed.deserialize(&mut *self.de).map(Some)
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.max_size
    }
}
