use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use serde::ser::{Impossible, Serialize, SerializeSeq, SerializeTuple, Serializer, Error as SerError};


/// Check if a value would be "wrapped" when serialized as XML. 
///
/// As a rule of thumb, if something **isn't** wrapped then the XML serialized
/// version would look similar to what you get from `Display`. Wrapped things 
/// typically look like `<foo>...</foo>`.
pub fn is_wrapped<S>(thing: &S) -> bool
where
    S: Serialize + ?Sized,
{
    let is_primitive = thing.serialize(WrapSafeDetector).is_ok();

    !is_primitive
}

/// A custom `Serializer` which will visit a type and determine if it would
/// be wrapped when serialized as XML. 
///
/// To work around the function signatures defined by the `Serializer` trait, if
/// something serializes with an error (i.e. returns `Err(_)`) it would be 
/// wrapped. Things which "serialize" fine aren't wrapped.
#[derive(Debug)]
struct WrapSafeDetector;

#[allow(unused_variables)]
impl Serializer for WrapSafeDetector {
    type Ok = ();
    type Error = DummyError;

    type SerializeSeq = DummySer;
    type SerializeTuple = DummySer;
    type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Impossible<Self::Ok, Self::Error>;
    type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_str(self, value: &str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(DummyError)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(DummyError)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(DummyError)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(DummyError)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // FIXME: Is a sequence of sequences okay here?
        Ok(DummySer)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        // FIXME: I think a tuple counts as a "primitive"... doesn't it?
        Ok(DummySer)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(DummyError)
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(DummyError)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(DummyError)
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Err(DummyError)
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(DummyError)
    }
}

#[derive(Clone, Copy, Debug)]
struct DummyError;

impl StdError for DummyError {
    fn description(&self) -> &'static str {
        "dummy error"
    }
}

impl Display for DummyError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ignore me")
    }
}

impl SerError for DummyError {
    fn custom<D: Display>(_: D) -> Self {
        DummyError
    }
}

#[derive(Clone, Copy, Debug)]
struct DummySer;

impl SerializeSeq for DummySer {
    type Ok = ();
    type Error = DummyError;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<Self::Ok, Self::Error> 
    where T: Serialize + ?Sized {
        if is_wrapped(elem) {
            Err(DummyError)
        } else {
            Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error>{ Ok(())}
}

impl SerializeTuple for DummySer {
    type Ok = ();
    type Error = DummyError;

    fn serialize_element<T>(&mut self, elem: &T) -> Result<Self::Ok, Self::Error> 
    where T: Serialize + ?Sized {
        if is_wrapped(elem) {
            Err(DummyError)
        } else {
            Ok(())
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error>{ Ok(())}
}


#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    macro_rules! prim_check {
        ($name:ident, $ty:path) => {
            #[test]
            fn $name() {
                let v: $ty = 0 as $ty;
                assert!(!is_wrapped(&v));
            }
        }
    }

    prim_check!(check_u8_is_primitive, u8);
    prim_check!(check_i8_is_primitive, i8);
    prim_check!(check_u16_is_primitive, u16);
    prim_check!(check_i16_is_primitive, i16);
    prim_check!(check_u32_is_primitive, u32);
    prim_check!(check_i32_is_primitive, i32);
    prim_check!(check_u64_is_primitive, u64);
    prim_check!(check_i64_is_primitive, i64);
    prim_check!(check_f32_is_primitive, f32);
    prim_check!(check_f64_is_primitive, f64);

    #[derive(Default, Serialize)]
    struct UnitStruct;

    #[derive(Default, Serialize)]
    struct Newtype(u32);

    #[derive(Default, Serialize)]
    struct TupleStruct(u32, bool);

    #[derive(Default, Serialize)]
    struct NormalStruct {
        x: u32,
        y: u32,
    }

    macro_rules! struct_check {
        ($name:ident, $ty:path) => {
            #[test]
            fn $name() {
                let value: $ty = Default::default();

                assert!(is_wrapped(&value));
            }
        }
    }

    struct_check!(check_unit_struct, UnitStruct);
    struct_check!(check_newtype_struct, Newtype);
    struct_check!(check_tuple_struct, TupleStruct);
    struct_check!(check_normal_struct, NormalStruct);

    #[derive(Serialize)]
    enum BoringEnum {
        A, 
        B,
    }

    #[derive(Serialize)]
    enum BasicRustEnum {
        A(u32), 
        B(f64),
    }

    #[derive(Serialize)]
    enum StructEnum {
        A{ x: u32, y: u32 },
    }

    macro_rules! enum_check {
        ($name:ident, $init:expr) => {
            #[test]
            fn $name() {
                let value = $init;
                assert!(is_wrapped(&value));
            }
        }
    }

    enum_check!(check_c_style_enum, BoringEnum::A);
    enum_check!(check_basic_rust_style_enum, BasicRustEnum::A(5));
    enum_check!(check_struct_enum, StructEnum::A{x: 1, y: 2});

    #[test]
    fn check_unit_is_primitive() {
        assert!(!is_wrapped(&()));
    }

    #[test]
    fn check_some_of_primitive_is_primitive() {
        assert!(!is_wrapped(&Some(42)));
    }

    #[test]
    fn check_none_is_primitive() {
        let value: Option<u32> = None;
        assert!(!is_wrapped(&value));
    }

    #[test]
    fn check_vec_of_primitives_isnt_wrapped() {
        let value = vec![1_u32, 2, 3, 4];
        assert!(!is_wrapped(&value));
    }
}