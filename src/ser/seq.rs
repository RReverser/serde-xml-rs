use super::Serializer;
use crate::error::{Error, Result};
use serde::ser::Serialize;
use std::io::Write;

pub struct SeqSeralizer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
}

impl<'ser, W: 'ser + Write> SeqSeralizer<'ser, W> {
    pub fn new(ser: &'ser mut Serializer<W>) -> Self {
        SeqSeralizer { ser }
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeSeq for SeqSeralizer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let seq_tag = self.ser.current_tag();
        value.serialize(&mut *self.ser)?;
        self.ser.open_tag(&seq_tag)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.ser.abandon_tag()?;
        Ok(())
    }
}
