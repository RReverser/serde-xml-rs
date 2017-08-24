use std::io::Write;
use serde::ser::{Error as SerError, Serialize, SerializeTuple, 
                 SerializeTupleStruct, SerializeTupleVariant};

use ser::Serializer;
use ser::helpers;
use error::Error;

pub struct Tuple<'a, W: 'a + Write> {
    name: Option<&'static str>,
    parent: &'a mut Serializer<W>,
}


impl<'w, W> Tuple<'w, W>
where
    W: 'w + Write,
{
    pub fn new(parent: &'w mut Serializer<W>) -> Self {
        Tuple { parent: parent, name: None }
    }

    pub fn new_with_name(parent: &'w mut Serializer<W>, name: &'static str) -> Self {
        Tuple { parent: parent, name: Some(name) }
    }
}

impl<'w, W> SerializeTuple for Tuple<'w, W>
where
    W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error> 
    where 
        T: Serialize + ?Sized 
    {
        if helpers::is_wrapped(value) {
            value.serialize(&mut *self.parent)
        } else {
            Err(SerError::custom(
                "Tuples can't contain primitive types. Please wrap primitives in a newtype.",
            ))
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}


impl<'w, W> SerializeTupleStruct for Tuple<'w, W>
where
    W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error> 
    where 
        T: Serialize + ?Sized 
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.parent.writer, "</{}>", self.name.expect("if we're serializing a tuple struct we should have been given a name"))?;
        Ok(())
    }
}

impl<'w, W> SerializeTupleVariant for Tuple<'w, W>
where
    W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok, Self::Error> 
    where 
        T: Serialize + ?Sized 
    {
        self.serialize_element(value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.parent.writer, "</{}>", self.name.expect("if we're serializing a tuple variant we should have been given a name"))?;
        Ok(())
    }
}
