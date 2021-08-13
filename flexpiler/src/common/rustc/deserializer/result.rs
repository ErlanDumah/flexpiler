use crate::common::rustc::block as block;
use crate::{error, Deserialization};
use crate::error::Trait;
use crate::parser;
use crate::reader;
use crate::reader::Error;


pub struct Result;


impl Result {
    fn context() -> error::Context {
        use std::str::FromStr;

        error::Context {
            trace: std::string::String::from("Result"),
        }
    }
}


impl<DataType, ErrorType> crate::deserializer::Trait<
    std::result::Result<DataType, ErrorType>,
    crate::common::rustc::deserializer::Context
> for Result
where DataType: Deserialization<crate::common::rustc::Format>,
      ErrorType: Deserialization<crate::common::rustc::Format> {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> std::result::Result<crate::deserializer::Result<std::result::Result<DataType, ErrorType>, crate::common::rustc::deserializer::Context>, error::Error>
    where ReaderType: reader::Trait {
        use crate::parser::Parse;

        let parse_identifier_result = match block::IdentifierWithVariableFinish::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(Result::context());
                return Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let result = match parse_identifier_result.identifier_string.as_str() {
            "Ok"
            | "Result::Ok" => {
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
                                    .propagate(Result::context());
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
                            .propagate(Result::context());
                        return Err(error);
                    }
                }

                let argument_0_result = match <DataType::Deserializer as crate::deserializer::Trait<DataType, crate::common::rustc::deserializer::Context>>::deserialize(reader_mut_ref) {
                    Ok(result) => result,
                    Err(error) => {
                        let error = error
                            .propagate(Result::context());
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
                                    .propagate(Result::context());
                                return Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = crate::error::MissingEnumArgumentClosure{
                            enum_declaration_found: parse_identifier_result.identifier_string,
                        };
                        let error = crate::Error::gen(missing_argument_closure)
                            .propagate(Result::context());
                        return Err(error);
                    },
                }

                let context = crate::common::rustc::deserializer::Context::Freestanding;
                crate::deserializer::Result {
                    data: std::result::Result::Ok(
                        argument_0_result.data,
                    ),
                    context
                }
            },

            "Err"
            | "Result::Err" => {
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
                                    .propagate(Result::context());
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
                            .propagate(Result::context());
                        return Err(error);
                    }
                }

                let argument_0_result = match <ErrorType::Deserializer as crate::deserializer::Trait<ErrorType, crate::common::rustc::deserializer::Context>>::deserialize(reader_mut_ref) {
                    Ok(result) => result,
                    Err(error) => {
                        let error = error
                            .propagate(Result::context());
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
                                    .propagate(Result::context());
                                return Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = crate::error::MissingEnumArgumentClosure{
                            enum_declaration_found: parse_identifier_result.identifier_string,
                        };
                        let error = crate::Error::gen(missing_argument_closure)
                            .propagate(Result::context());
                        return Err(error);
                    },
                }

                let context = crate::common::rustc::deserializer::Context::Freestanding;
                crate::deserializer::Result {
                    data: std::result::Result::Err(
                        argument_0_result.data,
                    ),
                    context
                }
            },

            _ => {
                let incompatible_declaration_found = error::IncompatibleEnumDeclaration {
                    enum_declaration_found: parse_identifier_result.identifier_string,
                    enum_declaration_expected_entries: error::ExpectedEntries::from(vec![
                        std::string::String::from("Ok"),
                        std::string::String::from("Result::Ok"),
                        std::string::String::from("Err"),
                        std::string::String::from("Result::Err"),
                    ]),
                };
                let error = crate::Error::gen(incompatible_declaration_found)
                    .propagate(Result::context());
                return Err(error);
            },
        };

        return Ok(result);
    }
}


impl<DataType, ErrorType> crate::Deserialization<crate::common::rustc::Format> for std::result::Result<DataType, ErrorType>
where DataType: crate::Deserialization<crate::common::rustc::Format>,
      ErrorType: crate::Deserialization<crate::common::rustc::Format>{
    type Deserializer = Result;
}
