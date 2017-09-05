use error::{ErrorKind, Error, Result};
use serde::ser::{self, Serialize};

pub struct StrSerializer {
    result: String,
}

pub fn serialize<T: Serialize>(v: T) -> Result<String> {
    let mut serializer = StrSerializer::new();
    v.serialize(&mut serializer)?;
    Ok(serializer.result)
}

impl StrSerializer {
    fn new() -> StrSerializer {
        StrSerializer { result: String::new() }
    }
}

// This serializer only allows for the serialization of str. Anything else results
// in an error.
impl<'a> ser::Serializer for &'a mut StrSerializer {
    type Ok = ();

    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        self.result = v.to_string();
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }


    fn serialize_none(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }


    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }


    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }


    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

impl<'a> ser::SerializeSeq for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

impl<'a> ser::SerializeMap for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

impl<'a> ser::SerializeStruct for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut StrSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(ErrorKind::NonPrimitiveKey.into())
    }

    fn end(self) -> Result<Self::Ok> {
        Err(ErrorKind::NonPrimitiveKey.into())
    }
}
