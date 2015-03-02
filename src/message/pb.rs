// This file is generated. Do not edit

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Header {
    message_length: ::std::option::Option<u32>,
    hmac_hash_function: ::std::option::Option<Header_HmacHashFunction>,
    hmac_signer: ::protobuf::SingularField<::std::string::String>,
    hmac_key_version: ::std::option::Option<u32>,
    hmac: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Header {
        static mut instance: ::protobuf::lazy::Lazy<Header> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Header,
        };
        unsafe {
            instance.get(|| {
                Header {
                    message_length: ::std::option::Option::None,
                    hmac_hash_function: ::std::option::Option::None,
                    hmac_signer: ::protobuf::SingularField::none(),
                    hmac_key_version: ::std::option::Option::None,
                    hmac: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 message_length = 1;

    pub fn clear_message_length(&mut self) {
        self.message_length = ::std::option::Option::None;
    }

    pub fn has_message_length(&self) -> bool {
        self.message_length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message_length(&mut self, v: u32) {
        self.message_length = ::std::option::Option::Some(v);
    }

    pub fn get_message_length<'a>(&self) -> u32 {
        self.message_length.unwrap_or(0)
    }

    // optional .message.Header.HmacHashFunction hmac_hash_function = 3;

    pub fn clear_hmac_hash_function(&mut self) {
        self.hmac_hash_function = ::std::option::Option::None;
    }

    pub fn has_hmac_hash_function(&self) -> bool {
        self.hmac_hash_function.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac_hash_function(&mut self, v: Header_HmacHashFunction) {
        self.hmac_hash_function = ::std::option::Option::Some(v);
    }

    pub fn get_hmac_hash_function<'a>(&self) -> Header_HmacHashFunction {
        self.hmac_hash_function.unwrap_or(Header_HmacHashFunction::MD5)
    }

    // optional string hmac_signer = 4;

    pub fn clear_hmac_signer(&mut self) {
        self.hmac_signer.clear();
    }

    pub fn has_hmac_signer(&self) -> bool {
        self.hmac_signer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac_signer(&mut self, v: ::std::string::String) {
        self.hmac_signer = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac_signer<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.hmac_signer.is_none() {
            self.hmac_signer.set_default();
        };
        self.hmac_signer.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac_signer(&mut self) -> ::std::string::String {
        self.hmac_signer.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hmac_signer<'a>(&'a self) -> &'a str {
        match self.hmac_signer.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional uint32 hmac_key_version = 5;

    pub fn clear_hmac_key_version(&mut self) {
        self.hmac_key_version = ::std::option::Option::None;
    }

    pub fn has_hmac_key_version(&self) -> bool {
        self.hmac_key_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac_key_version(&mut self, v: u32) {
        self.hmac_key_version = ::std::option::Option::Some(v);
    }

    pub fn get_hmac_key_version<'a>(&self) -> u32 {
        self.hmac_key_version.unwrap_or(0)
    }

    // optional bytes hmac = 6;

    pub fn clear_hmac(&mut self) {
        self.hmac.clear();
    }

    pub fn has_hmac(&self) -> bool {
        self.hmac.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hmac(&mut self, v: ::std::vec::Vec<u8>) {
        self.hmac = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hmac<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.hmac.is_none() {
            self.hmac.set_default();
        };
        self.hmac.as_mut().unwrap()
    }

    // Take field
    pub fn take_hmac(&mut self) -> ::std::vec::Vec<u8> {
        self.hmac.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hmac<'a>(&'a self) -> &'a [u8] {
        match self.hmac.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        if self.message_length.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.message_length = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.hmac_hash_function = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hmac_signer.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.hmac_key_version = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hmac.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.message_length.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.hmac_hash_function.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.hmac_signer.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.hmac_key_version.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.hmac.iter() {
            my_size += ::protobuf::rt::bytes_size(6, value.as_slice());
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message_length {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.hmac_hash_function {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.hmac_signer.as_ref() {
            try!(os.write_string(4, v.as_slice()));
        };
        if let Some(v) = self.hmac_key_version {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.hmac.as_ref() {
            try!(os.write_bytes(6, v.as_slice()));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Header>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Header {
    fn new() -> Header {
        Header::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Header>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "message_length",
                    Header::has_message_length,
                    Header::get_message_length,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "hmac_hash_function",
                    Header::has_hmac_hash_function,
                    Header::get_hmac_hash_function,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hmac_signer",
                    Header::has_hmac_signer,
                    Header::get_hmac_signer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "hmac_key_version",
                    Header::has_hmac_key_version,
                    Header::get_hmac_key_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "hmac",
                    Header::has_hmac,
                    Header::get_hmac,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Header>(
                    "Header",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.clear_message_length();
        self.clear_hmac_hash_function();
        self.clear_hmac_signer();
        self.clear_hmac_key_version();
        self.clear_hmac();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Header {
    fn eq(&self, other: &Header) -> bool {
        self.message_length == other.message_length &&
        self.hmac_hash_function == other.hmac_hash_function &&
        self.hmac_signer == other.hmac_signer &&
        self.hmac_key_version == other.hmac_key_version &&
        self.hmac == other.hmac &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Header_HmacHashFunction {
    MD5 = 0,
    SHA1 = 1,
}

impl ::protobuf::ProtobufEnum for Header_HmacHashFunction {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Header_HmacHashFunction> {
        match value {
            0 => ::std::option::Option::Some(Header_HmacHashFunction::MD5),
            1 => ::std::option::Option::Some(Header_HmacHashFunction::SHA1),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Header_HmacHashFunction>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Header_HmacHashFunction", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Header_HmacHashFunction {
}

#[derive(Clone,Default)]
pub struct Field {
    name: ::protobuf::SingularField<::std::string::String>,
    value_type: ::std::option::Option<Field_ValueType>,
    representation: ::protobuf::SingularField<::std::string::String>,
    value_string: ::protobuf::RepeatedField<::std::string::String>,
    value_bytes: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    value_integer: ::std::vec::Vec<i64>,
    value_double: ::std::vec::Vec<f64>,
    value_bool: ::std::vec::Vec<bool>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl Field {
    pub fn new() -> Field {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Field {
        static mut instance: ::protobuf::lazy::Lazy<Field> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Field,
        };
        unsafe {
            instance.get(|| {
                Field {
                    name: ::protobuf::SingularField::none(),
                    value_type: ::std::option::Option::None,
                    representation: ::protobuf::SingularField::none(),
                    value_string: ::protobuf::RepeatedField::new(),
                    value_bytes: ::protobuf::RepeatedField::new(),
                    value_integer: ::std::vec::Vec::new(),
                    value_double: ::std::vec::Vec::new(),
                    value_bool: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional .message.Field.ValueType value_type = 2;

    pub fn clear_value_type(&mut self) {
        self.value_type = ::std::option::Option::None;
    }

    pub fn has_value_type(&self) -> bool {
        self.value_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value_type(&mut self, v: Field_ValueType) {
        self.value_type = ::std::option::Option::Some(v);
    }

    pub fn get_value_type<'a>(&self) -> Field_ValueType {
        self.value_type.unwrap_or(Field_ValueType::STRING)
    }

    // optional string representation = 3;

    pub fn clear_representation(&mut self) {
        self.representation.clear();
    }

    pub fn has_representation(&self) -> bool {
        // This is a hack to the generated code to get tests to pass (but is
        // buggy across all protobuf fields, probably due to rust-protobuf
        // #ad76a527).
        self.representation.is_some() && self.representation == ::protobuf::SingularField::none()
    }

    // Param is passed by value, moved
    pub fn set_representation(&mut self, v: ::std::string::String) {
        self.representation = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_representation<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.representation.is_none() {
            self.representation.set_default();
        };
        self.representation.as_mut().unwrap()
    }

    // Take field
    pub fn take_representation(&mut self) -> ::std::string::String {
        self.representation.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_representation<'a>(&'a self) -> &'a str {
        match self.representation.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // repeated string value_string = 4;

    pub fn clear_value_string(&mut self) {
        self.value_string.clear();
    }

    // Param is passed by value, moved
    pub fn set_value_string(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.value_string = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value_string<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.value_string
    }

    // Take field
    pub fn take_value_string(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.value_string, ::protobuf::RepeatedField::new())
    }

    pub fn get_value_string<'a>(&'a self) -> &'a [::std::string::String] {
        self.value_string.as_slice()
    }

    // repeated bytes value_bytes = 5;

    pub fn clear_value_bytes(&mut self) {
        self.value_bytes.clear();
    }

    // Param is passed by value, moved
    pub fn set_value_bytes(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.value_bytes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value_bytes<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.value_bytes
    }

    // Take field
    pub fn take_value_bytes(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.value_bytes, ::protobuf::RepeatedField::new())
    }

    pub fn get_value_bytes<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        self.value_bytes.as_slice()
    }

    // repeated int64 value_integer = 6;

    pub fn clear_value_integer(&mut self) {
        self.value_integer.clear();
    }

    // Param is passed by value, moved
    pub fn set_value_integer(&mut self, v: ::std::vec::Vec<i64>) {
        self.value_integer = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value_integer<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.value_integer
    }

    // Take field
    pub fn take_value_integer(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.value_integer, ::std::vec::Vec::new())
    }

    pub fn get_value_integer<'a>(&'a self) -> &'a [i64] {
        self.value_integer.as_slice()
    }

    // repeated double value_double = 7;

    pub fn clear_value_double(&mut self) {
        self.value_double.clear();
    }

    // Param is passed by value, moved
    pub fn set_value_double(&mut self, v: ::std::vec::Vec<f64>) {
        self.value_double = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value_double<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<f64> {
        &mut self.value_double
    }

    // Take field
    pub fn take_value_double(&mut self) -> ::std::vec::Vec<f64> {
        ::std::mem::replace(&mut self.value_double, ::std::vec::Vec::new())
    }

    pub fn get_value_double<'a>(&'a self) -> &'a [f64] {
        self.value_double.as_slice()
    }

    // repeated bool value_bool = 8;

    pub fn clear_value_bool(&mut self) {
        self.value_bool.clear();
    }

    // Param is passed by value, moved
    pub fn set_value_bool(&mut self, v: ::std::vec::Vec<bool>) {
        self.value_bool = v;
    }

    // Mutable pointer to the field.
    pub fn mut_value_bool<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.value_bool
    }

    // Take field
    pub fn take_value_bool(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.value_bool, ::std::vec::Vec::new())
    }

    pub fn get_value_bool<'a>(&'a self) -> &'a [bool] {
        self.value_bool.as_slice()
    }
}

impl ::protobuf::Message for Field {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.value_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.representation.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.value_string));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.value_bytes));
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_int64_into(wire_type, is, &mut self.value_integer));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_double_into(wire_type, is, &mut self.value_double));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.value_bool));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, value.as_slice());
        };
        for value in self.value_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.representation.iter() {
            my_size += ::protobuf::rt::string_size(3, value.as_slice());
        };
        for value in self.value_string.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.value_bytes.iter() {
            my_size += ::protobuf::rt::bytes_size(5, value.as_slice());
        };
        if !self.value_integer.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(6, self.value_integer.as_slice());
        };
        if !self.value_double.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.value_double.len() as u32) + (self.value_double.len() * 8) as u32;
        };
        if !self.value_bool.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.value_bool.len() as u32) + (self.value_bool.len() * 1) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, v.as_slice()));
        };
        if let Some(v) = self.value_type {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.representation.as_ref() {
            try!(os.write_string(3, v.as_slice()));
        };
        for v in self.value_string.iter() {
            try!(os.write_string(4, v.as_slice()));
        };
        for v in self.value_bytes.iter() {
            try!(os.write_bytes(5, v.as_slice()));
        };
        if !self.value_integer.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(self.value_integer.as_slice())));
            for v in self.value_integer.iter() {
                try!(os.write_int64_no_tag(*v));
            };
        };
        if !self.value_double.is_empty() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.value_double.len() * 8) as u32));
            for v in self.value_double.iter() {
                try!(os.write_double_no_tag(*v));
            };
        };
        if !self.value_bool.is_empty() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.value_bool.len() * 1) as u32));
            for v in self.value_bool.iter() {
                try!(os.write_bool_no_tag(*v));
            };
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Field>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Field {
    fn new() -> Field {
        Field::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<Field>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Field::has_name,
                    Field::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "value_type",
                    Field::has_value_type,
                    Field::get_value_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "representation",
                    Field::has_representation,
                    Field::get_representation,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "value_string",
                    Field::get_value_string,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "value_bytes",
                    Field::get_value_bytes,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "value_integer",
                    Field::get_value_integer,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_f64_accessor(
                    "value_double",
                    Field::get_value_double,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "value_bool",
                    Field::get_value_bool,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Field>(
                    "Field",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Field {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value_type();
        self.clear_representation();
        self.clear_value_string();
        self.clear_value_bytes();
        self.clear_value_integer();
        self.clear_value_double();
        self.clear_value_bool();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Field {
    fn eq(&self, other: &Field) -> bool {
        self.name == other.name &&
        self.value_type == other.value_type &&
        self.representation == other.representation &&
        self.value_string == other.value_string &&
        self.value_bytes == other.value_bytes &&
        self.value_integer == other.value_integer &&
        self.value_double == other.value_double &&
        self.value_bool == other.value_bool &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Field {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug)]
pub enum Field_ValueType {
    STRING = 0,
    BYTES = 1,
    INTEGER = 2,
    DOUBLE = 3,
    BOOL = 4,
}

impl ::protobuf::ProtobufEnum for Field_ValueType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Field_ValueType> {
        match value {
            0 => ::std::option::Option::Some(Field_ValueType::STRING),
            1 => ::std::option::Option::Some(Field_ValueType::BYTES),
            2 => ::std::option::Option::Some(Field_ValueType::INTEGER),
            3 => ::std::option::Option::Some(Field_ValueType::DOUBLE),
            4 => ::std::option::Option::Some(Field_ValueType::BOOL),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Field_ValueType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Field_ValueType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Field_ValueType {
}

#[derive(Clone,Default)]
pub struct HekaMessage {
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<i64>,
    field_type: ::protobuf::SingularField<::std::string::String>,
    logger: ::protobuf::SingularField<::std::string::String>,
    severity: ::std::option::Option<i32>,
    payload: ::protobuf::SingularField<::std::string::String>,
    env_version: ::protobuf::SingularField<::std::string::String>,
    pid: ::std::option::Option<i32>,
    hostname: ::protobuf::SingularField<::std::string::String>,
    fields: ::protobuf::RepeatedField<Field>,
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl HekaMessage {
    pub fn new() -> HekaMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HekaMessage {
        static mut instance: ::protobuf::lazy::Lazy<HekaMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HekaMessage,
        };
        unsafe {
            instance.get(|| {
                HekaMessage {
                    uuid: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    field_type: ::protobuf::SingularField::none(),
                    logger: ::protobuf::SingularField::none(),
                    severity: ::std::option::Option::None,
                    payload: ::protobuf::SingularField::none(),
                    env_version: ::protobuf::SingularField::none(),
                    pid: ::std::option::Option::None,
                    hostname: ::protobuf::SingularField::none(),
                    fields: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid<'a>(&'a self) -> &'a [u8] {
        match self.uuid.as_ref() {
            Some(v) => v.as_slice(),
            None => [].as_slice(),
        }
    }

    // required int64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }

    // optional string type = 3;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        };
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type<'a>(&'a self) -> &'a str {
        match self.field_type.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string logger = 4;

    pub fn clear_logger(&mut self) {
        self.logger.clear();
    }

    pub fn has_logger(&self) -> bool {
        self.logger.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logger(&mut self, v: ::std::string::String) {
        self.logger = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_logger<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.logger.is_none() {
            self.logger.set_default();
        };
        self.logger.as_mut().unwrap()
    }

    // Take field
    pub fn take_logger(&mut self) -> ::std::string::String {
        self.logger.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_logger<'a>(&'a self) -> &'a str {
        match self.logger.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional int32 severity = 5;

    pub fn clear_severity(&mut self) {
        self.severity = ::std::option::Option::None;
    }

    pub fn has_severity(&self) -> bool {
        self.severity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_severity(&mut self, v: i32) {
        self.severity = ::std::option::Option::Some(v);
    }

    pub fn get_severity<'a>(&self) -> i32 {
        self.severity.unwrap_or(7i32)
    }

    // optional string payload = 6;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::string::String) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::string::String {
        self.payload.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_payload<'a>(&'a self) -> &'a str {
        match self.payload.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional string env_version = 7;

    pub fn clear_env_version(&mut self) {
        self.env_version.clear();
    }

    pub fn has_env_version(&self) -> bool {
        self.env_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_env_version(&mut self, v: ::std::string::String) {
        self.env_version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_env_version<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.env_version.is_none() {
            self.env_version.set_default();
        };
        self.env_version.as_mut().unwrap()
    }

    // Take field
    pub fn take_env_version(&mut self) -> ::std::string::String {
        self.env_version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_env_version<'a>(&'a self) -> &'a str {
        match self.env_version.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // optional int32 pid = 8;

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: i32) {
        self.pid = ::std::option::Option::Some(v);
    }

    pub fn get_pid<'a>(&self) -> i32 {
        self.pid.unwrap_or(0)
    }

    // optional string hostname = 9;

    pub fn clear_hostname(&mut self) {
        self.hostname.clear();
    }

    pub fn has_hostname(&self) -> bool {
        self.hostname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostname(&mut self, v: ::std::string::String) {
        self.hostname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostname<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostname.is_none() {
            self.hostname.set_default();
        };
        self.hostname.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostname(&mut self) -> ::std::string::String {
        self.hostname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostname<'a>(&'a self) -> &'a str {
        match self.hostname.as_ref() {
            Some(v) => v.as_slice(),
            None => "",
        }
    }

    // repeated .message.Field fields = 10;

    pub fn clear_fields(&mut self) {
        self.fields.clear();
    }

    // Param is passed by value, moved
    pub fn set_fields(&mut self, v: ::protobuf::RepeatedField<Field>) {
        self.fields = v;
    }

    // Mutable pointer to the field.
    pub fn mut_fields<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Field> {
        &mut self.fields
    }

    // Take field
    pub fn take_fields(&mut self) -> ::protobuf::RepeatedField<Field> {
        ::std::mem::replace(&mut self.fields, ::protobuf::RepeatedField::new())
    }

    pub fn get_fields<'a>(&'a self) -> &'a [Field] {
        self.fields.as_slice()
    }
}

impl ::protobuf::Message for HekaMessage {
    fn is_initialized(&self) -> bool {
        if self.uuid.is_none() {
            return false;
        };
        if self.timestamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.uuid.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.field_type.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.logger.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.severity = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.payload.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.env_version.set_default();
                    try!(is.read_string_into(tmp))
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.pid = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostname.set_default();
                    try!(is.read_string_into(tmp))
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.fields));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(1, value.as_slice());
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::string_size(3, value.as_slice());
        };
        for value in self.logger.iter() {
            my_size += ::protobuf::rt::string_size(4, value.as_slice());
        };
        for value in self.severity.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::string_size(6, value.as_slice());
        };
        for value in self.env_version.iter() {
            my_size += ::protobuf::rt::string_size(7, value.as_slice());
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.hostname.iter() {
            my_size += ::protobuf::rt::string_size(9, value.as_slice());
        };
        for value in self.fields.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(1, v.as_slice()));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.field_type.as_ref() {
            try!(os.write_string(3, v.as_slice()));
        };
        if let Some(v) = self.logger.as_ref() {
            try!(os.write_string(4, v.as_slice()));
        };
        if let Some(v) = self.severity {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_string(6, v.as_slice()));
        };
        if let Some(v) = self.env_version.as_ref() {
            try!(os.write_string(7, v.as_slice()));
        };
        if let Some(v) = self.pid {
            try!(os.write_int32(8, v));
        };
        if let Some(v) = self.hostname.as_ref() {
            try!(os.write_string(9, v.as_slice()));
        };
        for v in self.fields.iter() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HekaMessage>()
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HekaMessage {
    fn new() -> HekaMessage {
        HekaMessage::new()
    }

    #[allow(unused_unsafe,unused_mut)]
    fn descriptor_static(_: ::std::option::Option<HekaMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "uuid",
                    HekaMessage::has_uuid,
                    HekaMessage::get_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp",
                    HekaMessage::has_timestamp,
                    HekaMessage::get_timestamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "field_type",
                    HekaMessage::has_field_type,
                    HekaMessage::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "logger",
                    HekaMessage::has_logger,
                    HekaMessage::get_logger,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "severity",
                    HekaMessage::has_severity,
                    HekaMessage::get_severity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "payload",
                    HekaMessage::has_payload,
                    HekaMessage::get_payload,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "env_version",
                    HekaMessage::has_env_version,
                    HekaMessage::get_env_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "pid",
                    HekaMessage::has_pid,
                    HekaMessage::get_pid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hostname",
                    HekaMessage::has_hostname,
                    HekaMessage::get_hostname,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "fields",
                    HekaMessage::get_fields,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HekaMessage>(
                    "HekaMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HekaMessage {
    fn clear(&mut self) {
        self.clear_uuid();
        self.clear_timestamp();
        self.clear_field_type();
        self.clear_logger();
        self.clear_severity();
        self.clear_payload();
        self.clear_env_version();
        self.clear_pid();
        self.clear_hostname();
        self.clear_fields();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HekaMessage {
    fn eq(&self, other: &HekaMessage) -> bool {
        self.uuid == other.uuid &&
        self.timestamp == other.timestamp &&
        self.field_type == other.field_type &&
        self.logger == other.logger &&
        self.severity == other.severity &&
        self.payload == other.payload &&
        self.env_version == other.env_version &&
        self.pid == other.pid &&
        self.hostname == other.hostname &&
        self.fields == other.fields &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HekaMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x22, 0xc7, 0x01, 0x0a, 0x06, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x16, 0x0a, 0x0e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x6c,
    0x65, 0x6e, 0x67, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x41, 0x0a, 0x12, 0x68,
    0x6d, 0x61, 0x63, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x5f, 0x66, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x20, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x2e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x48, 0x6d, 0x61, 0x63, 0x48, 0x61, 0x73,
    0x68, 0x46, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x3a, 0x03, 0x4d, 0x44, 0x35, 0x12, 0x13,
    0x0a, 0x0b, 0x68, 0x6d, 0x61, 0x63, 0x5f, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x72, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x18, 0x0a, 0x10, 0x68, 0x6d, 0x61, 0x63, 0x5f, 0x6b, 0x65, 0x79, 0x5f,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0c, 0x0a,
    0x04, 0x68, 0x6d, 0x61, 0x63, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x25, 0x0a, 0x10, 0x48,
    0x6d, 0x61, 0x63, 0x48, 0x61, 0x73, 0x68, 0x46, 0x75, 0x6e, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12,
    0x07, 0x0a, 0x03, 0x4d, 0x44, 0x35, 0x10, 0x00, 0x12, 0x08, 0x0a, 0x04, 0x53, 0x48, 0x41, 0x31,
    0x10, 0x01, 0x22, 0xa2, 0x02, 0x0a, 0x05, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x12, 0x0c, 0x0a, 0x04,
    0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x34, 0x0a, 0x0a, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18,
    0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64, 0x2e, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x54, 0x79, 0x70, 0x65, 0x3a, 0x06, 0x53, 0x54, 0x52, 0x49, 0x4e, 0x47,
    0x12, 0x16, 0x0a, 0x0e, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x5f, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x12, 0x13,
    0x0a, 0x0b, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x62, 0x79, 0x74, 0x65, 0x73, 0x18, 0x05, 0x20,
    0x03, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x0d, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x69, 0x6e, 0x74,
    0x65, 0x67, 0x65, 0x72, 0x18, 0x06, 0x20, 0x03, 0x28, 0x03, 0x42, 0x02, 0x10, 0x01, 0x12, 0x18,
    0x0a, 0x0c, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x64, 0x6f, 0x75, 0x62, 0x6c, 0x65, 0x18, 0x07,
    0x20, 0x03, 0x28, 0x01, 0x42, 0x02, 0x10, 0x01, 0x12, 0x16, 0x0a, 0x0a, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x5f, 0x62, 0x6f, 0x6f, 0x6c, 0x18, 0x08, 0x20, 0x03, 0x28, 0x08, 0x42, 0x02, 0x10, 0x01,
    0x22, 0x45, 0x0a, 0x09, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a,
    0x06, 0x53, 0x54, 0x52, 0x49, 0x4e, 0x47, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x42, 0x59, 0x54,
    0x45, 0x53, 0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x4e, 0x54, 0x45, 0x47, 0x45, 0x52, 0x10,
    0x02, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x4f, 0x55, 0x42, 0x4c, 0x45, 0x10, 0x03, 0x12, 0x08, 0x0a,
    0x04, 0x42, 0x4f, 0x4f, 0x4c, 0x10, 0x04, 0x22, 0xc6, 0x01, 0x0a, 0x0b, 0x48, 0x65, 0x6b, 0x61,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61,
    0x6d, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x6c, 0x6f, 0x67, 0x67, 0x65, 0x72,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x08, 0x73, 0x65, 0x76, 0x65, 0x72, 0x69,
    0x74, 0x79, 0x18, 0x05, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x01, 0x37, 0x12, 0x0f, 0x0a, 0x07, 0x70,
    0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x12, 0x13, 0x0a, 0x0b,
    0x65, 0x6e, 0x76, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x0b, 0x0a, 0x03, 0x70, 0x69, 0x64, 0x18, 0x08, 0x20, 0x01, 0x28, 0x05, 0x12, 0x10,
    0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x09,
    0x12, 0x1e, 0x0a, 0x06, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x46, 0x69, 0x65, 0x6c, 0x64,
    0x4a, 0xd3, 0x13, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x2c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x00, 0x08, 0x0f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x0d,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x02, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x03, 0x02, 0x06, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x07, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x04, 0x07, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x04, 0x0b, 0x0c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x05, 0x04, 0x0d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x05, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x05, 0x0b, 0x0c, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x07, 0x02, 0x34, 0x22, 0x11, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x69, 0x6e,
    0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x1c,
    0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x32, 0x33, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x02, 0x44, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x06, 0x12, 0x03, 0x09, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x09, 0x1c, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x09, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x09,
    0x34, 0x43, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x07, 0x12, 0x03, 0x09, 0x3f, 0x42,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x02, 0x34, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x0a, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x0a, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0b,
    0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x0b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x0b, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x1c, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x04, 0x12, 0x03, 0x0c, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x0c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03,
    0x0c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x1c,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0c, 0x32, 0x33, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0f, 0x00, 0x1f, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x01, 0x01, 0x12, 0x03, 0x0f, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x01, 0x04, 0x00, 0x12,
    0x04, 0x10, 0x02, 0x16, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x10, 0x07, 0x10, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x11,
    0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11,
    0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x11,
    0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x12, 0x04,
    0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x12, 0x04,
    0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x12, 0x0e,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x13, 0x04, 0x10,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x04, 0x0b,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13, 0x0e, 0x0f,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x14, 0x04, 0x10, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x14, 0x04, 0x0a, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x14, 0x0e, 0x0f, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x15, 0x04, 0x10, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x15, 0x04, 0x08, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x01, 0x04, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x15, 0x0e, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x17, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x17, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x17, 0x18, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x17, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x18, 0x02, 0x3f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x18, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x18, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x18, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x08, 0x12, 0x03, 0x18, 0x2c, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x07, 0x12,
    0x03, 0x18, 0x37, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x19, 0x02,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x19, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x19, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x18, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x19, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x03, 0x12, 0x03, 0x1a, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12,
    0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1a,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x18, 0x24,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1a, 0x2a, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x1b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x1b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x1b, 0x18, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03,
    0x1b, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12, 0x03, 0x1c, 0x02, 0x3a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04, 0x12, 0x03, 0x1c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03, 0x1c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x1c, 0x18, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x05, 0x03, 0x12, 0x03, 0x1c, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05,
    0x08, 0x12, 0x03, 0x1c, 0x2c, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x05, 0x08, 0xe7,
    0x07, 0x00, 0x12, 0x03, 0x1c, 0x2d, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x2d, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x2d, 0x33, 0x0a, 0x12, 0x0a, 0x0b,
    0x04, 0x01, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x2d, 0x33,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1c,
    0x34, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x1d, 0x02, 0x3a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x1d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1d, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x1d, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x08,
    0x12, 0x03, 0x1d, 0x2c, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x12, 0x03, 0x1d, 0x2d, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x06, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x03, 0x1d, 0x2d, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x06,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x2d, 0x33, 0x0a, 0x12, 0x0a, 0x0b, 0x04,
    0x01, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x2d, 0x33, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x34,
    0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x07, 0x12, 0x03, 0x1e, 0x02, 0x3a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x04, 0x12, 0x03, 0x1e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x07, 0x05, 0x12, 0x03, 0x1e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x1e, 0x18, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07,
    0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x07, 0x08, 0x12,
    0x03, 0x1e, 0x2c, 0x39, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x1e, 0x2d, 0x38, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x07, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x1e, 0x2d, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01, 0x02, 0x07, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x2d, 0x33, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x01,
    0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x2d, 0x33, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x01, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x34, 0x38,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x21, 0x00, 0x2c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x22, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x22, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x14, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22, 0x22, 0x23, 0x0a, 0x2b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x02, 0x24, 0x22, 0x1e, 0x20, 0x6e, 0x61, 0x6e,
    0x6f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x55,
    0x4e, 0x49, 0x58, 0x20, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x23, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x23, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23,
    0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x24, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x24, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x24, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x25, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x25,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x25, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x14, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x25, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x26, 0x02, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x26, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05,
    0x12, 0x03, 0x26, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x26, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x08, 0x12, 0x03, 0x26, 0x24, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x07, 0x12, 0x03, 0x26, 0x2f, 0x30, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x27, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x05, 0x12, 0x03, 0x27, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x27, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03, 0x27,
    0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x28, 0x02, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x28, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x28, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12,
    0x03, 0x29, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03, 0x29,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x29, 0x14, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x29, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x2a, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x08, 0x04, 0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x05,
    0x12, 0x03, 0x2a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12, 0x03,
    0x2a, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x2a, 0x22,
    0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x2b, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x09, 0x06, 0x12, 0x03, 0x2b, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x2b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09,
    0x03, 0x12, 0x03, 0x2b, 0x22, 0x24,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
