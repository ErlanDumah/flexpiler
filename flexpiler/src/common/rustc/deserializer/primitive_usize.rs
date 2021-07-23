use std::str::FromStr;

use crate::common::rustc::{block as block, Format};
use crate::{error, Error};
use crate::error::Trait;
use crate::parser;
use crate::reader;


// Proxy class
pub struct PrimitiveUSize;


fn context() -> error::Context {
    error::Context {
        trace: String::from("usize"),
    }
}


impl crate::deserializer::Trait<
    usize,
    crate::common::rustc::deserializer::Context
> for PrimitiveUSize {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> Result<crate::deserializer::Result<usize, crate::common::rustc::deserializer::Context>, error::Error>
    where ReaderType: reader::Trait {
        use crate::parser::Parse;

        let parse_number_result = match block::Number::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(context());
                return Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let data = match usize::from_str(parse_number_result.string.as_str()) {
            Err(parse_int_error) => {
                let error = error::Error::gen(parse_int_error)
                    .propagate(context());
                return Err(error);
            },
            Ok(value) => value,
        };
        let context = parse_number_result.finish.into();

        return Ok(crate::deserializer::Result {
            data,
            context,
        });
    }
}


impl crate::Deserialization<crate::common::rustc::Format> for usize {
    type Deserializer = PrimitiveUSize;
}
