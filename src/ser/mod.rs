extern crate xmltree;

use error::{Error, ErrorKind, Result};
use serde::ser::{self, Serialize};

use std::io::Write;

mod str_serializer;

/// A convenience method for serializing some object to a buffer.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde;
/// # extern crate serde_xml_rs;
/// # use serde_xml_rs::to_writer;
/// #[derive(Serialize)]
/// struct Person {
///   name: String,
///   age: u32,
/// }
///
/// # fn main() {
/// let mut buffer = Vec::new();
/// let joe = Person {name: "Joe".to_string(), age: 42};
///
/// to_writer(&mut buffer, &joe).unwrap();
///
/// let serialized = String::from_utf8(buffer).unwrap();
/// println!("{}", serialized);
/// # }
/// ```
pub fn to_writer<W: Write, S: Serialize>(writer: W, value: &S) -> Result<()> {
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    let ref tree = serializer.current_path[0];
    tree.write(writer);
    Ok(())
}

/// A convenience method for serializing some object to a string.
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_derive;
/// # extern crate serde;
/// # extern crate serde_xml_rs;
/// # use serde_xml_rs::to_string;
/// #[derive(Serialize)]
/// struct Person {
///   name: String,
///   age: u32,
/// }
///
/// # fn main() {
///
/// let joe = Person {name: "Joe".to_string(), age: 42};
/// let serialized = to_string(&joe).unwrap();
/// println!("{}", serialized);
/// # }
/// ```
pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;

    let mut buffer = Vec::new();
    let ref tree = serializer.current_path[0];
    tree.write(&mut buffer);
    let result = String::from_utf8(buffer).unwrap();
    Ok(result)
}

pub struct Serializer {
    // holds a list of elements from the root node down
    // to the element we're currently serializing.
    pub current_path: Vec<xmltree::Element>,
    serialize_names: bool,
    serializing_name: bool,
    first_struct: bool,
}

impl Serializer {
    pub fn new() -> Serializer {
        Serializer {
            current_path: Vec::new(),
            serialize_names: false,
            serializing_name: true,
            first_struct: true,
        }
    }

    // Take the last element off current_path and make it a child
    // of the new last element in current_path.
    fn pop_current_path(&mut self) {
        let child_element = self.current_path.pop().unwrap();
        let parent_element = self.current_path.pop();
        if let Some(mut parent) = parent_element {
            parent.children.push(child_element);
            self.current_path.push(parent);
        } else {
            self.current_path.push(child_element);
        }
    }

    // Make sure we serialize item names if we're in a sequence.
    fn serialize_item_name_if_wrapped(&mut self, name: &str) {
        if self.serialize_names || self.first_struct {
            self.first_struct = false;
            self.serializing_name = true;
            let name_element = xmltree::Element::new(name);
            self.current_path.push(name_element);
        }
    }

    fn pop_current_path_if_wrapped(&mut self) {
        if self.serializing_name {
            self.serializing_name = false;
            self.pop_current_path();
        }
    }
}


impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        let mut element = self.current_path.pop().unwrap();
        if v {
            element.text = Some("true".to_string())
        } else {
            element.text = Some("false".to_string())
        }
        self.current_path.push(element);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok> {
        let mut element = self.current_path.pop().unwrap();
        element.text = Some(v.to_string());
        self.current_path.push(element);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok> {
        let mut element = self.current_path.pop().unwrap();
        element.text = Some(v.to_string());
        self.current_path.push(element);
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok> {
        let mut element = self.current_path.pop().unwrap();
        element.text = Some(v.to_string());
        self.current_path.push(element);
        Ok(())
    }

    // Serialize a char as a single-character string.
    fn serialize_char(self, v: char) -> Result<Self::Ok> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok> {
        let mut element = self.current_path.pop().unwrap();
        element.text = Some(v.to_string());
        self.current_path.push(element);
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok> {
        Err(
            ErrorKind::UnsupportedOperation("serialize_bytes".to_string()).into(),
        )
    }

    // An absent optional is represented with an empty element.
    fn serialize_none(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok> {
        self.serialize_unit()
    }

    // Represent unit variants as value
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.serialize_str(variant)?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // Represent E::N(a) as <N>a</N>, wrap it in the struct name if serializing a sequence.
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_item_name_if_wrapped(name);

        let variant_element = xmltree::Element::new(variant);
        self.current_path.push(variant_element);

        value.serialize(&mut *self)?;

        self.pop_current_path();
        self.pop_current_path_if_wrapped();

        Ok(())
    }

    // We want to wrap each element in a sequence in the name of its parent type, so turn
    // on serialization of the type names. For example, vec[]
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.serialize_names = true;
        Ok(self)
    }

    // Tuples look just like sequences in XML.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Ditto for tuple structs, but serialize the name.
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        let name_element = xmltree::Element::new(name);
        self.current_path.push(name_element);
        self.serialize_seq(Some(len))?;
        self.pop_current_path();
        Ok(self)
    }

    // Tuple variants are represented as <name>sequence of data</name>
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        let name_element = xmltree::Element::new(name);
        self.current_path.push(name_element);
        variant.serialize(&mut *self)?;
        self.pop_current_path();
        Ok(self)
    }

    // Maps are represented as <k>v</k><k>v</k>... So nothing is done here, but is handled
    // in the SerializeMap impl below.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(self)
    }

    // Structs look just like maps in xml.
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_item_name_if_wrapped(name);
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        let name_element = xmltree::Element::new(name);
        self.current_path.push(name_element);
        variant.serialize(&mut *self)?;
        Ok(self)
    }
}

// The following 7 impls deal with the serialization of compound types like
// sequences and maps. Serialization of such types is begun by a Serializer
// method and followed by zero or more calls to serialize individual elements of
// the compound type and one call to end the compound type.
//
// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<Self::Ok> {
        self.serialize_names = false;
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.pop_current_path();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.pop_current_path();
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // XML only allows for keys to be strings, so we use a special serializer
    // which only accepts str below.
    fn serialize_key<T>(&mut self, key: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let name = str_serializer::serialize(key)?;
        let key_element = xmltree::Element::new(&name);
        self.current_path.push(key_element);
        Ok(())
    }


    fn serialize_value<T>(&mut self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;
        self.pop_current_path();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let element = xmltree::Element::new(key);
        self.current_path.push(element);
        value.serialize(&mut **self)?;
        self.pop_current_path();
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.pop_current_path_if_wrapped();
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok> {
        self.pop_current_path();
        Ok(())
    }
}
