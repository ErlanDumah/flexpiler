use crate::common::rustc::{block as block, Format};
use crate::{deserializer, Error};
use crate::error;
use crate::parser;
use crate::reader;
use crate::error::Context;
use crate::format::Trait;


// Proxy class
pub struct PrimitiveI32;


fn context() -> error::Context {
    use std::str::FromStr;

    error::Context {
        trace: String::from("i32"),
    }
}


impl crate::deserializer::Trait<
    i32,
    crate::common::rustc::deserializer::Context
> for PrimitiveI32 {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> std::result::Result<crate::deserializer::Result<i32, crate::common::rustc::deserializer::Context>, Error>
    where ReaderType: crate::reader::Trait {
        use crate::error::Trait;
        use crate::parser::Parse;
        use std::str::FromStr;

        let parse_number_result = match block::Number::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(context());
                return Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let data = match i32::from_str(parse_number_result.string.as_str()) {
            Err(parse_int_error) => {
                let error = error::Error::gen(parse_int_error)
                    .propagate(context());
                return Err(error);
            },
            Ok(value) => value,
        };
        let context: crate::common::rustc::deserializer::Context = parse_number_result.finish.into();

        let result = crate::deserializer::Result {
            data,
            context,
        };

        return Ok(result);
    }
}


impl crate::Deserialization<crate::common::rustc::Format> for i32 {
    type Deserializer = PrimitiveI32;
}
