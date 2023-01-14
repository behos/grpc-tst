// This file is generated by rust-protobuf 2.28.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `calculator.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_28_0;

#[derive(PartialEq,Clone,Default)]
pub struct Add {
    // message fields
    pub amount: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Add {
    fn default() -> &'a Add {
        <Add as ::protobuf::Message>::default_instance()
    }
}

impl Add {
    pub fn new() -> Add {
        ::std::default::Default::default()
    }

    // int32 amount = 1;


    pub fn get_amount(&self) -> i32 {
        self.amount
    }
    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = v;
    }
}

impl ::protobuf::Message for Add {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.amount = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(1, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.amount != 0 {
            os.write_int32(1, self.amount)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Add {
        Add::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "amount",
                |m: &Add| { &m.amount },
                |m: &mut Add| { &mut m.amount },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Add>(
                "Add",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Add {
        static instance: ::protobuf::rt::LazyV2<Add> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Add::new)
    }
}

impl ::protobuf::Clear for Add {
    fn clear(&mut self) {
        self.amount = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Add {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Add {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Mul {
    // message fields
    pub amount: i32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Mul {
    fn default() -> &'a Mul {
        <Mul as ::protobuf::Message>::default_instance()
    }
}

impl Mul {
    pub fn new() -> Mul {
        ::std::default::Default::default()
    }

    // int32 amount = 1;


    pub fn get_amount(&self) -> i32 {
        self.amount
    }
    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = v;
    }
}

impl ::protobuf::Message for Mul {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.amount = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(1, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.amount != 0 {
            os.write_int32(1, self.amount)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Mul {
        Mul::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "amount",
                |m: &Mul| { &m.amount },
                |m: &mut Mul| { &mut m.amount },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Mul>(
                "Mul",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Mul {
        static instance: ::protobuf::rt::LazyV2<Mul> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Mul::new)
    }
}

impl ::protobuf::Clear for Mul {
    fn clear(&mut self) {
        self.amount = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Mul {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mul {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Op {
    // message oneof groups
    pub op: ::std::option::Option<Op_oneof_op>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Op {
    fn default() -> &'a Op {
        <Op as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum Op_oneof_op {
    add(Add),
    mul(Mul),
}

impl Op {
    pub fn new() -> Op {
        ::std::default::Default::default()
    }

    // .calculator.Add add = 1;


    pub fn get_add(&self) -> &Add {
        match self.op {
            ::std::option::Option::Some(Op_oneof_op::add(ref v)) => v,
            _ => <Add as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_add(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_add(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(Op_oneof_op::add(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_add(&mut self, v: Add) {
        self.op = ::std::option::Option::Some(Op_oneof_op::add(v))
    }

    // Mutable pointer to the field.
    pub fn mut_add(&mut self) -> &mut Add {
        if let ::std::option::Option::Some(Op_oneof_op::add(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(Op_oneof_op::add(Add::new()));
        }
        match self.op {
            ::std::option::Option::Some(Op_oneof_op::add(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_add(&mut self) -> Add {
        if self.has_add() {
            match self.op.take() {
                ::std::option::Option::Some(Op_oneof_op::add(v)) => v,
                _ => panic!(),
            }
        } else {
            Add::new()
        }
    }

    // .calculator.Mul mul = 2;


    pub fn get_mul(&self) -> &Mul {
        match self.op {
            ::std::option::Option::Some(Op_oneof_op::mul(ref v)) => v,
            _ => <Mul as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_mul(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_mul(&self) -> bool {
        match self.op {
            ::std::option::Option::Some(Op_oneof_op::mul(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_mul(&mut self, v: Mul) {
        self.op = ::std::option::Option::Some(Op_oneof_op::mul(v))
    }

    // Mutable pointer to the field.
    pub fn mut_mul(&mut self) -> &mut Mul {
        if let ::std::option::Option::Some(Op_oneof_op::mul(_)) = self.op {
        } else {
            self.op = ::std::option::Option::Some(Op_oneof_op::mul(Mul::new()));
        }
        match self.op {
            ::std::option::Option::Some(Op_oneof_op::mul(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_mul(&mut self) -> Mul {
        if self.has_mul() {
            match self.op.take() {
                ::std::option::Option::Some(Op_oneof_op::mul(v)) => v,
                _ => panic!(),
            }
        } else {
            Mul::new()
        }
    }
}

impl ::protobuf::Message for Op {
    fn is_initialized(&self) -> bool {
        if let Some(Op_oneof_op::add(ref v)) = self.op {
            if !v.is_initialized() {
                return false;
            }
        }
        if let Some(Op_oneof_op::mul(ref v)) = self.op {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.op = ::std::option::Option::Some(Op_oneof_op::add(is.read_message()?));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.op = ::std::option::Option::Some(Op_oneof_op::mul(is.read_message()?));
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let ::std::option::Option::Some(ref v) = self.op {
            match v {
                &Op_oneof_op::add(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
                &Op_oneof_op::mul(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.op {
            match v {
                &Op_oneof_op::add(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
                &Op_oneof_op::mul(ref v) => {
                    os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Op {
        Op::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Add>(
                "add",
                Op::has_add,
                Op::get_add,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, Mul>(
                "mul",
                Op::has_mul,
                Op::get_mul,
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Op>(
                "Op",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Op {
        static instance: ::protobuf::rt::LazyV2<Op> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Op::new)
    }
}

impl ::protobuf::Clear for Op {
    fn clear(&mut self) {
        self.op = ::std::option::Option::None;
        self.op = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Op {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Op {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x10calculator.proto\x12\ncalculator\"\x1d\n\x03Add\x12\x16\n\x06amoun\
    t\x18\x01\x20\x01(\x05R\x06amount\"\x1d\n\x03Mul\x12\x16\n\x06amount\x18\
    \x01\x20\x01(\x05R\x06amount\"T\n\x02Op\x12#\n\x03add\x18\x01\x20\x01(\
    \x0b2\x0f.calculator.AddH\0R\x03add\x12#\n\x03mul\x18\x02\x20\x01(\x0b2\
    \x0f.calculator.MulH\0R\x03mulB\x04\n\x02op2:\n\nCalculator\x12,\n\x04ca\
    lc\x12\x0e.calculator.Op\x1a\x0e.calculator.Op\"\0(\x010\x01b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
