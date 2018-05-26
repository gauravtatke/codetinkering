//#[macro_use]
//extern crate serde_derive;
// extern crate serde;

use serde::ser::{self, Serialize, Serializer};
use std;
use std::fmt::{self, Display};


// struct Logon {
//     message_type: u32,
//     hearbeat_interval: u32,
//     username: String,
//     password: String
// }

struct FixSerializer {
    msg: String
}

#[derive(Clone, Debug, PartialEq)]
pub enum FixError {
    Message(String) 
}

impl ser::Error for FixError {
    fn custom<T: Display>(msg: T) -> Self {
        FixError::Message(msg.to_string())
    }
}

impl std::error::Error for FixError {
    fn description(&self) -> &str {
        match *self {
            FixError::Message(ref msg) => msg,
            //FixError::Eof => "unexpected end of input",
        }
    }
}

impl Display for FixError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

//type Result<T> = std::result::Result<T, FixError>;

pub fn to_string<T>(value: &T) -> Result<String, FixError> 
    where T: Serialize {
        let mut serializer = FixSerializer { msg: String::new()};
        value.serialize(&mut serializer)?;
        Ok(serializer.msg)
    }

impl<'a> Serializer for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    //copied from json library
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), Self::Error> {
        self.msg += if v { "true"} else { "false" };
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<(), Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<(), Self::Error> {
        self.serialize_i64(v as i64)
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<(), Self::Error> {
        self.msg += &v.to_string();
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<(), Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<(), Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<(), Self::Error> {
        self.msg += &v.to_string();
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<(), Self::Error> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<(), Self::Error> {
        self.msg += &v.to_string();
        Ok(())
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<(), Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<(), Self::Error> {
        //self.output += "\"";
        self.msg += v;
        //self.output += "\"";
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<(), Self::Error> {
        // use serde::ser::SerializeSeq;
        // let mut seq = self.serialize_seq(Some(v.len()))?;
        // for byte in v {
        //     seq.serialize_element(byte)?;
        // }
        // seq.end()
        Ok(())
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<(), Self::Error> {
        //self.output += "null"; //in FIX it should remove the empty tag
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(self, _name: &'static str, variant_index: u32, _variant: &'static str) -> Result<(), Self::Error> {
        self.serialize_u32(variant_index)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32,
        variant: &'static str, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        //self.output += "{";
        variant.serialize(&mut *self)?;
        self.msg += "=";
        value.serialize(&mut *self)?;
        //self.output += "}";
        Ok(())
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(n) => self.msg += &n.to_string(),
            None => self.msg += "errorlen",
        }
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        //self.msg += "{";
        variant.serialize(&mut *self)?;
        //self.output += ":[";
        Ok(self)
    }

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        //self.output += "{";
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        //self.output += "{";
        variant.serialize(&mut *self)?;
        //self.output += ":{";
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
impl<'a> ser::SerializeSeq for &'a mut FixSerializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = FixError;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with("|") {
            self.msg += "|";
        }
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<(), Self::Error> {
        //self.output += "]";
        Ok(())
    }
}


impl<'a> ser::SerializeTuple for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with('[') {
            self.msg += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.msg += "]";
        Ok(())
    }
}


// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with('[') {
            self.msg += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.msg += "]";
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with('[') {
            self.msg += ",";
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.msg += "]}";
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with('{') {
            self.msg += ",";
        }
        key.serialize(&mut **self)
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        self.msg += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.msg += "}";
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with("|") {
            self.msg += "|";
        }
        key.serialize(&mut **self)?;
        self.msg += "=";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.msg += "|";
        Ok(())
    }
}


// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut FixSerializer {
    type Ok = ();
    type Error = FixError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
        where T: ?Sized + Serialize
    {
        if !self.msg.ends_with('{') {
            self.msg += ",";
        }
        key.serialize(&mut **self)?;
        self.msg += ":";
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Self::Error> {
        self.msg += "}}";
        Ok(())
    }
}


