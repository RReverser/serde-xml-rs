use std::io::Write;

use serde::ser::Serialize;

use super::Serializer;
use crate::error::{Error, Result};

pub struct TupleSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
    first: bool,
}

impl<'ser, W: 'ser + Write> TupleSerializer<'ser, W> {
    pub fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        Self {
            ser,
            must_close_tag,
            first: true,
        }
    }

    fn serialize_item<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if self.first {
            self.first = false;
        } else {
            self.ser.characters(" ")?;
        }
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn after_items(self) -> Result<()> {
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeTupleVariant for TupleSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<()> {
        self.ser.end_tag()?;
        self.after_items()
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeTupleStruct for TupleSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<()> {
        self.after_items()
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeTuple for TupleSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item(value)
    }

    fn end(self) -> Result<()> {
        self.after_items()
    }
}
