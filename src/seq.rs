use std::io::Read;
use xml::reader::XmlEvent;
use {Deserializer, Error};
use serde::de::{self, DeserializeSeed};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: &'a mut Deserializer<R>,
    max_size: Option<usize>,
    expected_name: Option<String>
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(de: &'a mut Deserializer<R>, max_size: Option<usize>) -> Self {
        let expected_name = if de.unset_map_value() {
            debug_expect!(de.peek(), Ok(&XmlEvent::StartElement { ref name, .. }) => {
                Some(name.local_name.clone())
            })
        } else {
            None
        };
        SeqAccess {
            de: de,
            max_size: max_size,
            expected_name: expected_name
        }
    }
}

impl<'de, 'a, R: 'a + Read> de::SeqAccess<'de> for SeqAccess<'a, R> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Error> {
        match self.max_size.as_mut() {
            Some(&mut 0) => {
                return Ok(None);
            }
            Some(max_size) => {
                *max_size -= 1;
            }
            None => {}
        }
        let more = match (self.de.peek()?, self.expected_name.as_ref()) {
            (&XmlEvent::StartElement { ref name, .. }, Some(expected_name)) => {
                &name.local_name == expected_name
            }
            (&XmlEvent::EndElement { .. }, None) => false,
            (_, Some(_)) => false,
            (_, None) => true
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
