use std::str::FromStr;

use crate::common::rustc::{block as block, Format};
use crate::{error, Error};
use crate::error::{Trait, Context};
use crate::parser;
use crate::reader;


// Proxy class
pub struct PrimitiveUSize;


impl crate::identity::Trait for usize {
    fn definition() -> String {
        return String::from("usize");
    }
}


impl crate::deserializer::Trait<
    usize,
    crate::common::rustc::deserializer::Context,
    crate::common::rustc::error::Source
> for PrimitiveUSize {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> crate::deserializer::Result<usize, crate::common::rustc::deserializer::Context, crate::Error<crate::common::rustc::error::Source>>
    where ReaderType: reader::Trait {
        use crate::deserializer::context::Trait;
        use crate::error::Trait as ErrorTrait;
        use crate::error::propagation::Trait as PropagationTrait;
        use crate::parser::Parse;

        let parse_number_result = match block::Number::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(<Self as crate::deserializer::context::Trait<usize, crate::common::rustc::Format>>::context_general());
                return crate::deserializer::Result::Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let data = match usize::from_str(parse_number_result.string.as_str()) {
            Err(parse_int_error) => {
                let error = error::Error::gen(parse_int_error)
                    .propagate(<Self as crate::deserializer::context::Trait<usize, crate::common::rustc::Format>>::context_general());
                return crate::deserializer::Result::Err(error);
            },
            Ok(value) => value,
        };
        let context = parse_number_result.finish.into();

        return crate::deserializer::Result::DataFound {
            data,
            context,
        };
    }
}


impl crate::Deserialization<crate::common::rustc::Format> for usize {
    type Deserializer = PrimitiveUSize;
}
