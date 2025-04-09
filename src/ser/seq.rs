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
        let must_close_tag = self.ser.build_start_tag()?;
        value.serialize(&mut *self.ser)?;
        if must_close_tag {
            self.ser.end_tag()?;
            self.ser.reopen_tag()?;
        }
        Ok(())
    }

    fn end(self) -> Result<()> {
        // TODO: Commenting this out fixes it, but unsure why
        // self.ser.abandon_tag()?;
        Ok(())
    }
}
