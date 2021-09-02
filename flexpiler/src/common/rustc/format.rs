use crate::common::rustc;


pub struct Format;


impl crate::format::Trait for Format {
    type DeserializerContext = rustc::deserializer::Context;
    type ErrorSource = rustc::error::Source;
}
