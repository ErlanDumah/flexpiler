use crate::common::rustc::{block as block, Format};
use crate::{deserializer, Error};
use crate::error;
use crate::parser;
use crate::reader;
use crate::error::Context;
use crate::format::Trait;


// Proxy class
pub struct PrimitiveI32;


impl crate::identity::Trait for i32 {
    fn definition() -> std::string::String {
        return std::string::String::from("i32");
    }
}


impl crate::deserializer::Trait<
    i32,
    crate::common::rustc::Format
> for PrimitiveI32 {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> crate::deserializer::Result<i32, crate::common::rustc::deserializer::Context, crate::Error<crate::common::rustc::error::Source>>
    where ReaderType: crate::reader::Trait {
        use crate::error::Trait as ErrorTrait;
        use crate::error::propagation::Trait as PropagationTrait;
        use crate::parser::Parse;
        use std::str::FromStr;

        let parse_number_result = match block::Number::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(<Self as crate::deserializer::context::Trait<i32, crate::common::rustc::Format>>::context());
                return crate::deserializer::Result::Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let data = match i32::from_str(parse_number_result.string.as_str()) {
            Err(parse_int_error) => {
                let error = error::Error::gen(parse_int_error)
                    .propagate(<Self as crate::deserializer::context::Trait<i32, crate::common::rustc::Format>>::context());
                return crate::deserializer::Result::Err(error);
            },
            Ok(value) => value,
        };
        let context: crate::common::rustc::deserializer::Context = parse_number_result.finish.into();

        return crate::deserializer::Result::DataFound {
            data,
            context,
        };
    }
}


impl crate::Deserialization<crate::common::rustc::Format> for i32 {
    type Deserializer = PrimitiveI32;
}
