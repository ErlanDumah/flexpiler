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


impl crate::identity::Trait for std::string::String {
    fn definition() -> std::string::String {
        return std::string::String::from("String");
    }
}


impl crate::deserializer::Trait<
    std::string::String,
    crate::common::rustc::Format
> for String {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> crate::deserializer::Result<std::string::String, crate::common::rustc::deserializer::Context, crate::Error<crate::common::rustc::error::Source>>
    where ReaderType: reader::Trait {
        use crate::error::Trait as ErrorTrait;
        use crate::error::propagation::Trait as PropagationTrait;
        use crate::parser::Parse;

        let (string_data, string_finish) = match block::String::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(context());
                return crate::deserializer::Result::Err(error);
            },
            Ok(block::string::Result::NoDataFound { finish }) => {
                return crate::deserializer::Result::NoDataFound { context: finish.into() };
            },
            Ok(block::string::Result::DataFound { data, finish }) => {
                (data, finish)
            },
        };

        return crate::deserializer::Result::DataFound{
            data: string_data,
            context: string_finish.into(),
        };
    }
}


impl crate::Deserialization<crate::common::rustc::Format> for std::string::String {
    type Deserializer = String;
}
