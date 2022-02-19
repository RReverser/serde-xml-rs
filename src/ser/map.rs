use super::{plain::to_plain_string, Serializer};
use crate::error::{Error, Result};
use log::debug;
use serde::ser::Serialize;
use std::io::Write;

pub struct MapSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
}

impl<'ser, W: 'ser + Write> MapSerializer<'ser, W> {
    pub fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        MapSerializer {
            ser,
            must_close_tag,
        }
    }
}

impl<'ser, W: Write> serde::ser::SerializeMap for MapSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.ser.open_tag(&to_plain_string(key)?)?;
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

pub struct StructSerializer<'ser, W: 'ser + Write> {
    ser: &'ser mut Serializer<W>,
    must_close_tag: bool,
}

impl<'ser, W: 'ser + Write> StructSerializer<'ser, W> {
    pub fn new(ser: &'ser mut Serializer<W>, must_close_tag: bool) -> Self {
        StructSerializer {
            ser,
            must_close_tag,
        }
    }

    fn serialize_struct_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if key.starts_with("@") {
            debug!("attribute {}", key);
            self.ser.add_attr(&key[1..], to_plain_string(value)?)
        } else if key == "$value" {
            self.ser.build_start_tag()?;
            debug!("body");
            value.serialize(&mut *self.ser)?;
            Ok(())
        } else {
            self.ser.build_start_tag()?;
            self.ser.open_tag(key)?;
            debug!("field {}", key);
            value.serialize(&mut *self.ser)?;
            debug!("end field");
            Ok(())
        }
    }

    fn after_fields(self) -> Result<()> {
        self.ser.build_start_tag()?;
        self.ser.end_tag()?;
        if self.must_close_tag {
            self.ser.end_tag()?;
        }
        Ok(())
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeStruct for StructSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_field(key, value)
    }

    fn end(self) -> Result<()> {
        self.after_fields()
    }
}

impl<'ser, W: 'ser + Write> serde::ser::SerializeStructVariant for StructSerializer<'ser, W> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_struct_field(key, value)
    }

    fn end(self) -> Result<()> {
        self.after_fields()
    }
}
