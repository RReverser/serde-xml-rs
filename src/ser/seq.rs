use std::io::Write;
use serde::ser::{Error as SerError, Serialize, SerializeSeq};

use ser::Serializer;
use ser::helpers;
use error::{Error, Result};

pub struct Seq<'a, W: 'a + Write> {
    parent: &'a mut Serializer<W>,
}


impl<'w, W> Seq<'w, W>
where
    W: 'w + Write,
{
    pub fn new(parent: &'w mut Serializer<W>) -> Seq<'w, W> {
        Seq { parent }
    }
}

impl<'a, W: Write> SerializeSeq for Seq<'a, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: Serialize + ?Sized,
    {
        if helpers::is_wrapped(value) {
            value.serialize(&mut *self.parent)
        } else {
            Err(SerError::custom(
                "Cannot serialize a sequence of primitives. Please wrap them in newtypes",
            ))
        }
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}
