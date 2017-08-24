use std::io::Write;
use std::fmt::Display;

use serde::ser::{self, Impossible, Serialize};
use xml::writer::{XmlEvent};

use ser::Serializer;
use error::{Error, ErrorKind, Result};

/// An implementation of `SerializeMap` for serializing to XML.
pub struct Map<'w, W>
where
    W: 'w + Write,
{
    parent: &'w mut Serializer<W>,
}

impl<'w, W> Map<'w, W>
where
    W: 'w + Write,
{
    pub fn new(parent: &'w mut Serializer<W>) -> Map<'w, W> {
        Map { parent }
    }
}

impl<'w, W> ser::SerializeMap for Map<'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _: &T) -> Result<()> {
        panic!("impossible to serialize the key on its own, please use serialize_entry()")
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.parent)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_entry<K: ?Sized + Serialize, V: ?Sized + Serialize>(
        &mut self,
        key: &K,
        value: &V,
    ) -> Result<()> {
        let key = key.serialize(&mut MapKeySerializer)?;
        self.parent.write_wrapped(key.as_str(), value)?;
        Ok(())
    }
}

pub struct MapKeySerializer;

#[allow(unused_variables)]
impl<'w> ser::Serializer for &'w mut MapKeySerializer
{
    type Ok = String;
    type Error = Error;
    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        Ok(v.to_string())
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_some<T: ? Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_newtype_struct<T: ? Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_newtype_variant<T: ? Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok> where
        T: Serialize {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant> {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }

    fn collect_str<T: ? Sized>(self, value: &T) -> Result<Self::Ok> where
        T: Display {
        Err(ErrorKind::Custom("wrong type".to_string()).into())
    }
}

/// An implementation of `SerializeStruct` for serializing to XML.
pub struct Struct<'w, W>
where
    W: 'w + Write,
{
    parent: &'w mut Serializer<W>,
}

impl<'w, W> Struct<'w, W>
where
    W: 'w + Write,
{
    pub fn new(parent: &'w mut Serializer<W>) -> Struct<'w, W> {
        Struct { parent }
    }
}

impl<'w, W> ser::SerializeStruct for Struct<'w, W>
where
    W: 'w + Write,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.parent.write_wrapped(key, value)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.parent.writer.write(XmlEvent::end_element())?;
        Ok(())
    }
}
