use crate::common::rustc::block as block;
use crate::error;
use crate::error::Trait;
use crate::parser;
use crate::reader;


fn context() -> error::Context {
    use std::str::FromStr;

    error::Context {
        trace: std::string::String::from("String"),
    }
}


pub struct String;


impl crate::deserializer::Trait<
    std::string::String,
    crate::common::rustc::deserializer::Context
> for String {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> Result<crate::deserializer::Result<std::string::String, crate::common::rustc::deserializer::Context>, error::Error>
    where ReaderType: reader::Trait {
        use crate::parser::Parse;

        let parse_string_result = match block::String::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(context());
                return Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let context = parse_string_result.finish.into();
        let data = parse_string_result.string;

        return Ok(crate::deserializer::Result{
            data,
            context,
        });
    }
}


impl crate::Deserialization<crate::common::rustc::Format> for std::string::String {
    type Deserializer = String;
}
