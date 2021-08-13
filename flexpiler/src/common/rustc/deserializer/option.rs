use crate::common::rustc::block as block;
use crate::{error, Deserialization};
use crate::error::Trait;
use crate::parser;
use crate::reader;


pub struct Option;


impl Option {
    fn context() -> error::Context {
        use std::str::FromStr;

        error::Context {
            trace: std::string::String::from("Option"),
        }
    }
}


impl<DataType> crate::deserializer::Trait<
    std::option::Option<DataType>,
    crate::common::rustc::deserializer::Context
> for Option
where DataType: Deserialization<crate::common::rustc::Format> {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> std::result::Result<crate::deserializer::Result<std::option::Option<DataType>, crate::common::rustc::deserializer::Context>, error::Error>
    where ReaderType: reader::Trait {
        use crate::parser::Parse;

        let parse_identifier_result = match block::IdentifierWithVariableFinish::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(Option::context());
                return Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let result = match parse_identifier_result.identifier_string.as_str() {
            "None"
            | "Option::None" => {
                let context: crate::common::rustc::deserializer::Context = parse_identifier_result.finish.into();
                crate::deserializer::Result {
                    data: std::option::Option::None,
                    context,
                }
            },

            "Some"
            | "Option::Some" => {
                let context: crate::common::rustc::deserializer::Context = parse_identifier_result.finish.into();
                match context {
                    crate::common::rustc::deserializer::Context::Freestanding => {
                        match crate::parser::parse::<
                            crate::common::rustc::block::ArgumentStart,
                            ReaderType
                        >(reader_mut_ref) {
                            Ok(result) => {},
                            Err(parser_error) => {
                                let error = crate::error::Error::gen(parser_error)
                                    .propagate(Option::context());
                                return Err(error);
                            },
                        }
                    },
                    crate::common::rustc::deserializer::Context::ArgumentStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = crate::error::IncompatibleEnumDataType {
                            enum_declaration_found: parse_identifier_result.identifier_string,
                            enum_data_type_expected: crate::error::incompatibleenumdataType::EnumDataType::Argument,
                            context_found: context,
                        };
                        let error = crate::Error::gen(incompatible_enum_data_type)
                            .propagate(Option::context());
                        return Err(error);
                    }
                }

                let argument_0_result = match <DataType::Deserializer as crate::deserializer::Trait<DataType, crate::common::rustc::deserializer::Context>>::deserialize(reader_mut_ref) {
                    Ok(result) => result,
                    Err(error) => {
                        let error = error
                            .propagate(Option::context());
                        return Err(error);
                    },
                };

                let argument_0_context = argument_0_result.context;
                match argument_0_context {
                    crate::common::rustc::deserializer::Context::ArgumentEnd => {},
                    crate::common::rustc::deserializer::Context::Freestanding => {
                        match crate::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                            Ok(_) => {
                            }
                            Err(parser_error) => {
                                let error = crate::error::Error::gen(parser_error)
                                    .propagate(Option::context());
                                return Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = crate::error::MissingEnumArgumentClosure{
                            enum_declaration_found: parse_identifier_result.identifier_string,
                        };
                        let error = crate::Error::gen(missing_argument_closure)
                            .propagate(Option::context());
                        return Err(error);
                    },
                }

                let context = crate::common::rustc::deserializer::Context::Freestanding;
                crate::deserializer::Result {
                    data: std::option::Option::Some(
                        argument_0_result.data,
                    ),
                    context
                }
            },

            _ => {
                let incompatible_declaration_found = error::IncompatibleEnumDeclaration {
                    enum_declaration_found: parse_identifier_result.identifier_string,
                    enum_declaration_expected_entries: error::ExpectedEntries::from(vec![
                        std::string::String::from("Some"),
                        std::string::String::from("Option::Some"),
                        std::string::String::from("None"),
                        std::string::String::from("Option::None"),
                    ]),
                };
                let error = crate::Error::gen(incompatible_declaration_found)
                    .propagate(Option::context());
                return Err(error);
            },
        };

        return Ok(result);
    }
}


impl<DataType> crate::Deserialization<crate::common::rustc::Format> for std::option::Option<DataType>
where DataType: crate::Deserialization<crate::common::rustc::Format> {
    type Deserializer = Option;
}
