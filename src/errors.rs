use std::fmt::Display;
use std::io;

use serde::{ser, de};

error_chain! {
    errors {
        Custom(message: String) {
            description("custom error")
            display("{}", message)
        }

        DeserializeBool {
            description("failed to deserialize bool")
        }

        DeserializeDouble {
            description("failed to deserialize double")
        }

        DeserializeFloat {
            description("failed to deserialize float")
        }

        DeserializeInteger(bits: u8) {
            description("failed to deserialize signed integer")
            display("failed to deserialize {}-bit signed integer", bits)
        }

        DeserializeOpaque {
            description("failed to deserialize opaque data")
        }

        DeserializeOption {
            description("failed to deserialize option")
        }

        DeserializeString {
            description("failed to deserialize string")
        }

        DeserializeStruct(name: String) {
            description("failed to deserialize struct")
            display("failed to deserialize struct: {}", name)
        }

        DeserializeStructField(struct_name: String, field_name: String) {
            description("failed to deserialize struct field")
            display(
                "failed to deserialize struct field: {}::{}",
                struct_name,
                field_name,
            )
        }

        DeserializeUnknownType {
            description("can't deserialize unknown type")
        }

        DeserializeUnsignedInteger(bits: u8) {
            description("failed to deserialize unsigned integer")
            display("failed to deserialize {}-bit unsigned integer", bits)
        }

        InvalidBool {
            description("deserialized an invalid bool")
        }

        InvalidDataType(type_name: String) {
            description("data type not supported")
            display("data type not supported: {}", type_name)
        }

        InvalidOption {
            description("deserialized an invalid option")
        }

        InvalidInteger(bits: u8, value: i32) {
            description("deserialized invalid signed integer")
            display(
                "deserialized invalid {}-bit signed integer: {}",
                bits,
                value,
            )
        }

        InvalidUnsignedInteger(bits: u8, value: u32) {
            description("deserialized invalid unsigned integer")
            display(
                "deserialized invalid {}-bit unsigned integer: {}",
                bits,
                value,
            )
        }

        SequenceTooLong(length: usize) {
            description("sequence is too long to be serialized")
            display("sequence is too long to be serialized: {}", length)
        }

        SerializeBool(value: bool) {
            description("failed to serialize bool")
            display("failed to serialize bool: {}", value)
        }

        SerializeDouble(value: f64) {
            description("failed to serialize double")
            display("failed to serialize double: {}", value)
        }

        SerializeEnum(name: String, variant: String) {
            description("failed to serialize enum variant")
            display("failed to serialize enum variant: {}::{}", name, variant)
        }

        SerializeFloat(value: f32) {
            description("failed to serialize float")
            display("failed to serialize float: {}", value)
        }

        SerializeHyperInteger(value: i64) {
            description("failed to serialize hyper integer")
            display("failed to serialize hyper integer: {}", value)
        }

        SerializeInteger(value: i32) {
            description("failed to serialize integer")
            display("failed to serialize integer: {}", value)
        }

        SerializeNone {
            description("failed to serialize 'none' optional variant")
        }

        SerializeOpaque(size: usize) {
            description("failed to serialize opaque")
            display("failed to serialize opaque data of {} bytes", size)
        }

        SerializeSome {
            description("failed to serialize some optional data")
        }

        SerializeString(string: String) {
            description("failed to serialize string")
            display("failed to serialize string: {}", string)
        }

        SerializeSequenceElement(index: usize) {
            description("failed to serialize sequence element")
            display("failed to serialize sequence element {}", index)
        }

        SerializeSequenceFatalError {
            description("fatal failure while serializing sequence")
        }

        SerializeSequenceLength(length: usize) {
            description("failed to serialize sequence length")
            display("failed to serialize sequence length: {}", length)
        }

        SerializeSequenceWithUnknownLength {
            description("can't serialize sequence with unknown length")
        }

        SerializeStruct(name: String) {
            description("failed to serialize struct")
            display("failed to serialize struct: {}", name)
        }

        SerializeStructFatalError(name: String) {
            description("fatal failure while serializing struct")
            display("fatal failure while serializing struct: {}", name)
        }

        SerializeStructField(struct_name: String, field_name: String) {
            description("failed to serialize struct field")
            display(
                "failed to serialize struct field: {}::{}",
                struct_name,
                field_name,
            )
        }

        SerializeUnionVariant(name: String, variant: String) {
            description("failed to serialize union variant")
            display("failed to serialize union variant: {}::{}", name, variant)
        }

        SerializeUnsignedHyperInteger(value: u64) {
            description("failed to serialize unsigned hyper integer")
            display("failed to serialize unsigned hyper integer: {}", value)
        }

        SerializeUnsignedInteger(value: u32) {
            description("failed to serialize unsigned integer")
            display("failed to serialize unsigned integer: {}", value)
        }

        StringIsNotAscii(string: String) {
            description("string is not ASCII encoded")
            display("string is not ASCII encoded: {}", string)
        }

        StringIsTooLong(string: String) {
            description("string is too long")
            display(
                "string is too long (maximum length is {} bytes): {}",
                u32::max_value(),
                string,
            )
        }
    }

    foreign_links {
        Io(io::Error);
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        ErrorKind::Custom(message.to_string()).into()
    }
}

impl de::Error for Error {
    fn custom<T: Display>(message: T) -> Self {
        ErrorKind::Custom(message.to_string()).into()
    }
}
