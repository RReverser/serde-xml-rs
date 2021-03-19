use std::io::Read;

use serde::de;
use xml::reader::XmlEvent;

use de::ChildDeserializer;
use error::{Error, Result};

pub struct SeqAccess<'a, R: 'a + Read> {
    de: ChildDeserializer<'a, R>,
    max_size: Option<usize>,
    seq_type: SeqType,
}

pub enum SeqType {
    /// Sequence is of elements with the same name.
    ByElementName { expected_name: String },
    /// Sequence is of all elements/text at current depth.
    AllMembers,
}

impl<'a, R: 'a + Read> SeqAccess<'a, R> {
    pub fn new(mut de: ChildDeserializer<'a, R>, max_size: Option<usize>) -> Self {
        let seq_type = if de.unset_map_value() {
            debug_expect!(de.peek(), Ok(&XmlEvent::StartElement { ref name, .. }) => {
                SeqType::ByElementName { expected_name: name.local_name.clone() }
            })
        } else {
            SeqType::AllMembers
        };
        SeqAccess {
            de,
            max_size,
            seq_type,
        }
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

        let mut local_depth = 0;

        match &self.seq_type {
            SeqType::ByElementName { expected_name } => loop {
                let next_element = self.de.peek()?;

                match next_element {
                    XmlEvent::StartElement { name, .. } if &name.local_name == expected_name => {
                        self.de.set_map_value();
                        return seed.deserialize(&mut self.de).map(Some);
                    }
                    XmlEvent::StartElement { .. } => {
                        self.de.buffered_reader.skip();
                        local_depth += 1;
                    }
                    XmlEvent::EndElement { .. } => {
                        if local_depth == 0 {
                            return Ok(None);
                        } else {
                            local_depth -= 1;
                            self.de.buffered_reader.skip();
                        }
                    }
                    XmlEvent::EndDocument => {
                        return Ok(None);
                    }
                    _ => {
                        self.de.buffered_reader.skip();
                    }
                }
            },
            SeqType::AllMembers => {
                let next_element = self.de.peek()?;

                match next_element {
                    XmlEvent::EndElement { .. } | XmlEvent::EndDocument => return Ok(None),
                    _ => {
                        return seed.deserialize(&mut self.de).map(Some);
                    }
                }
            }
        }
    }

    fn size_hint(&self) -> Option<usize> {
        self.max_size
    }
}
