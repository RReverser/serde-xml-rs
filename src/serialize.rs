use std::io::Write;
use serde::ser::{self, Impossible, Serialize};
use error::Error;


/// An XML Serializer.
pub struct Serializer<W>
    where W: Write
{
    writer: W,
}

impl<W> Serializer<W>
    where W: Write
{
    pub fn new(writer: W) -> Self {
        Self { writer: writer }
    }

    fn into_inner(self) -> W {
        self.writer
    }
}


#[allow(unused_variables)]
impl<'w, W> ser::Serializer for &'w mut Serializer<W>
    where W: Write
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Impossible<Self::Ok, Self::Error>;
    type SerializeTuple = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = CompoundSerializer<'w, W>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(mut self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            write!(self.writer, "true")?;
        } else {
            write!(self.writer, "false")?;
        }

        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        write!(self.writer, "{}", value)
            .map(|_| ())
            .map_err(|e| e.into())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_unit_variant(self,
                              name: &'static str,
                              variant_index: usize,
                              variant: &'static str)
                              -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(self,
                                                       name: &'static str,
                                                       value: &T)
                                                       -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(self,
                                                        name: &'static str,
                                                        variant_index: usize,
                                                        variant: &'static str,
                                                        value: &T)
                                                        -> Result<Self::Ok, Self::Error> {
        unimplemented!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_seq_fixed_size(self, size: usize) -> Result<Self::SerializeSeq, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_struct(self,
                              name: &'static str,
                              len: usize)
                              -> Result<Self::SerializeTupleStruct, Self::Error> {
        unimplemented!()
    }

    fn serialize_tuple_variant(self,
                               name: &'static str,
                               variant_index: usize,
                               variant: &'static str,
                               len: usize)
                               -> Result<Self::SerializeTupleVariant, Self::Error> {
        unimplemented!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        unimplemented!()
    }

    fn serialize_struct(self,
                        name: &'static str,
                        len: usize)
                        -> Result<Self::SerializeStruct, Self::Error> {
        write!(self.writer, "<{}>", name)?;
        Ok(CompoundSerializer {
               parent: self,
               name: name,
           })
    }

    fn serialize_struct_variant(self,
                                name: &'static str,
                                variant_index: usize,
                                variant: &'static str,
                                len: usize)
                                -> Result<Self::SerializeStructVariant, Self::Error> {
        unimplemented!()
    }
}

pub struct CompoundSerializer<'w, W>
    where W: 'w + Write
{
    parent: &'w mut Serializer<W>,
    name: &'w str,
}

impl<'w, W> ser::SerializeStruct for CompoundSerializer<'w, W>
    where W: 'w + Write
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self,
                                              key: &'static str,
                                              value: &T)
                                              -> Result<(), Self::Error> {
        write!(self.parent.writer, "<{}>", key)?;
        value.serialize(&mut *self.parent)?;
        write!(self.parent.writer, "</{}>", key)?;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        write!(self.parent.writer, "</{}>", self.name).map_err(|e| e.into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use serde::Serializer as SerSerializer;
    use serde::ser::SerializeStruct;

    #[test]
    fn test_serialize_bool() {
        let inputs = vec![(true, "true"), (false, "false")];

        for (src, should_be) in inputs {
            let mut buffer = Vec::new();

            {
                let mut ser = Serializer::new(&mut buffer);
                ser.serialize_bool(src).unwrap();
            }

            let got = String::from_utf8(buffer).unwrap();
            assert_eq!(got, should_be);
        }
    }

    #[test]
    fn test_start_serialize_struct() {
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let _ = ser.serialize_struct("foo", 0).unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, "<foo>");
    }

    #[test]
    fn test_serialize_struct_field() {
        let mut buffer = Vec::new();

        {
            let mut ser = Serializer::new(&mut buffer);
            let mut struct_ser = CompoundSerializer {
                parent: &mut ser,
                name: "baz",
            };
            struct_ser.serialize_field("foo", "bar").unwrap();
        }

        let got = String::from_utf8(buffer).unwrap();
        assert_eq!(got, "<foo>bar</foo>");
    }
}
