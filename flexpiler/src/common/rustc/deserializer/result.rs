use crate::common::rustc::block as block;
use crate::deserializer;
use crate::{error, Deserialization};
use crate::error::Trait;
use crate::parser;
use crate::reader;
use crate::reader::Error;


pub struct Result;


impl<DataType, ErrorType> crate::identity::Trait for std::result::Result<DataType, ErrorType>
where DataType: crate::identity::Trait,
      ErrorType: crate::identity::Trait
{
    fn definition() -> String {
        return std::string::String::from(format!("Result<{}, {}>",
            <DataType as crate::identity::Trait>::definition(),
            <ErrorType as crate::identity::Trait>::definition()
        ))
    }
}


impl<DataType, ErrorType> crate::deserializer::Trait<
    std::result::Result<DataType, ErrorType>,
    crate::common::rustc::Format
> for Result
where DataType: Deserialization<crate::common::rustc::Format>
                + crate::identity::Trait,
      ErrorType: Deserialization<crate::common::rustc::Format>
                 + crate::identity::Trait
{
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> crate::deserializer::Result<std::result::Result<DataType, ErrorType>, crate::common::rustc::deserializer::Context, crate::Error<crate::common::rustc::error::Source>>
    where ReaderType: reader::Trait {
        use crate::deserializer::context::Trait as DeserializerContextTrait;
        use crate::error::Trait as ErrorTrait;
        use crate::error::propagation::Trait as PropagationTrait;
        use crate::parser::Parse;

        let (identifier_data, identifier_finish) = match block::Identifier::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                return crate::deserializer::Result::Err(error);
            },
            Ok(block::identifier::Result::NoDataFound { finish }) => {
                return crate::deserializer::Result::NoDataFound {
                    context: finish.into()
                };
            },
            Ok(block::identifier::Result::DataFound { data, finish }) => {
                (data, finish)
            },
        };

        let result = match identifier_data.as_str() {
            "Ok"
            | "Result::Ok" => {
                let context: crate::common::rustc::deserializer::Context = identifier_finish.into();
                match context {
                    crate::common::rustc::deserializer::Context::Freestanding => {
                        match crate::parser::parse::<
                            crate::common::rustc::block::ArgumentStart,
                            ReaderType
                        >(reader_mut_ref) {
                            Ok(result) => {},
                            Err(parser_error) => {
                                let error = crate::error::Error::gen(parser_error)
                                    .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                                return crate::deserializer::Result::Err(error);
                            },
                        }
                    },
                    crate::common::rustc::deserializer::Context::ArgumentStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = crate::common::rustc::error::IncompatibleEnumDataType {
                            enum_declaration_found: identifier_data,
                            enum_data_type_expected: crate::common::rustc::error::incompatibleenumdataType::EnumDataType::Argument,
                            context_found: context,
                        };
                        let error = crate::Error::gen(incompatible_enum_data_type)
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return crate::deserializer::Result::Err(error);
                    }
                }

                let (argument_0_data, argument_0_context) = match <DataType::Deserializer as crate::deserializer::Trait<DataType, crate::common::rustc::Format>>::deserialize(reader_mut_ref) {
                    deserializer::Result::DataFound{data, context} => (data, context),
                    deserializer::Result::NoDataFound { context } => {
                        let unexpected_no_content = crate::common::rustc::error::MissingEnumArgument {
                            enum_declaration_found: identifier_data,
                            argument_type_expected: DataType::definition(),
                        };
                        let error = crate::Error::gen(unexpected_no_content)
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return deserializer::Result::Err(error);
                    }
                    deserializer::Result::Err(error) => {
                        let error = error
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return crate::deserializer::Result::Err(error);
                    },
                };

                match argument_0_context {
                    crate::common::rustc::deserializer::Context::ArgumentEnd => {},
                    crate::common::rustc::deserializer::Context::Freestanding => {
                        match crate::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                            Ok(_) => {
                            }
                            Err(parser_error) => {
                                let error = crate::error::Error::gen(parser_error)
                                    .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                                return crate::deserializer::Result::Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = crate::common::rustc::error::MissingEnumArgumentClosure{
                            enum_declaration_found: identifier_data,
                        };
                        let error = crate::Error::gen(missing_argument_closure)
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return crate::deserializer::Result::Err(error);
                    },
                }

                let context = crate::common::rustc::deserializer::Context::Freestanding;
                crate::deserializer::Result::DataFound {
                    data: std::result::Result::Ok(argument_0_data),
                    context
                }
            },

            "Err"
            | "Result::Err" => {
                let context: crate::common::rustc::deserializer::Context = identifier_finish.into();
                match context {
                    crate::common::rustc::deserializer::Context::Freestanding => {
                        match crate::parser::parse::<
                            crate::common::rustc::block::ArgumentStart,
                            ReaderType
                        >(reader_mut_ref) {
                            Ok(result) => {},
                            Err(parser_error) => {
                                let error = crate::error::Error::gen(parser_error)
                                    .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                                return crate::deserializer::Result::Err(error);
                            },
                        }
                    },
                    crate::common::rustc::deserializer::Context::ArgumentStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = crate::common::rustc::error::IncompatibleEnumDataType {
                            enum_declaration_found: identifier_data,
                            enum_data_type_expected: crate::common::rustc::error::incompatibleenumdataType::EnumDataType::Argument,
                            context_found: context,
                        };
                        let error = crate::Error::gen(incompatible_enum_data_type)
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return crate::deserializer::Result::Err(error);
                    }
                }

                let (argument_0_data, argument_0_context) = match <ErrorType::Deserializer as crate::deserializer::Trait<ErrorType, crate::common::rustc::Format>>::deserialize(reader_mut_ref) {
                    deserializer::Result::DataFound{ data, context } => (data, context),
                    deserializer::Result::NoDataFound { context } => {
                        let unexpected_no_content = crate::common::rustc::error::MissingEnumArgument {
                            enum_declaration_found: identifier_data,
                            argument_type_expected: ErrorType::definition(),
                        };
                        let error = crate::Error::gen(unexpected_no_content)
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return deserializer::Result::Err(error);
                    }
                    deserializer::Result::Err(error) => {
                        let error = error
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return crate::deserializer::Result::Err(error);
                    },
                };

                match argument_0_context {
                    crate::common::rustc::deserializer::Context::ArgumentEnd => {},
                    crate::common::rustc::deserializer::Context::Freestanding => {
                        match crate::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                            Ok(_) => {
                            }
                            Err(parser_error) => {
                                let error = crate::error::Error::gen(parser_error)
                                    .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                                return crate::deserializer::Result::Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = crate::common::rustc::error::MissingEnumArgumentClosure{
                            enum_declaration_found: identifier_data,
                        };
                        let error = crate::Error::gen(missing_argument_closure)
                            .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                        return crate::deserializer::Result::Err(error);
                    },
                }

                let context = crate::common::rustc::deserializer::Context::Freestanding;
                crate::deserializer::Result::DataFound {
                    data: std::result::Result::Err(
                        argument_0_data,
                    ),
                    context
                }
            },

            _ => {
                let incompatible_declaration_found = crate::common::rustc::error::IncompatibleEnumDeclaration {
                    enum_declaration_found: identifier_data,
                    enum_declaration_expected_entries: error::ExpectedEntries::from(vec![
                        std::string::String::from("Ok"),
                        std::string::String::from("Result::Ok"),
                        std::string::String::from("Err"),
                        std::string::String::from("Result::Err"),
                    ]),
                };
                let error = crate::Error::gen(incompatible_declaration_found)
                    .propagate(<Result as crate::deserializer::context::Trait<std::result::Result<DataType, ErrorType>, crate::common::rustc::Format>>::context());
                return crate::deserializer::Result::Err(error);
            },
        };

        return result;
    }
}


impl<DataType, ErrorType> crate::Deserialization<crate::common::rustc::Format> for std::result::Result<DataType, ErrorType>
where DataType: crate::Deserialization<crate::common::rustc::Format>
                + crate::identity::Trait,
      ErrorType: crate::Deserialization<crate::common::rustc::Format>
                 + crate::identity::Trait
{
    type Deserializer = Result;
}
