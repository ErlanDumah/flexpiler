use crate::common::rustc::block;
use crate::deserializer;
use crate::error;
use crate::parser;
use crate::reader;
use crate::deserializer::Result::DataFound;


pub struct Option;


impl<DataType> crate::identity::Trait for std::option::Option<DataType>
where DataType: crate::identity::Trait {
    fn definition() -> String {
        std::string::String::from(format!("Option<{}>", DataType::definition()))
    }
}


impl<DataType> crate::deserializer::Trait<
    std::option::Option<DataType>,
    crate::common::rustc::deserializer::Context,
    crate::common::rustc::error::Source,
> for Option
where DataType: crate::Deserialization<crate::common::rustc::Format>
                + crate::identity::Trait
{
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> crate::deserializer::Result<std::option::Option<DataType>, crate::common::rustc::deserializer::Context, error::Error<crate::common::rustc::error::Source>>
    where ReaderType: reader::Trait {
        use crate::deserializer::context::Trait as DeserializerContextTrait;
        use crate::error::propagation::Trait as PropagationTrait;
        use crate::error::Trait as ErrorTrait;
        use crate::parser::Parse;

        let parse_identifier_result = match block::IdentifierWithVariableFinish::parse(reader_mut_ref) {
            Err(parser_error) => {
                let error = error::Error::gen(parser_error)
                    .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                return deserializer::Result::Err(error);
            },
            Ok(parser_result) => parser_result,
        };

        let result = match parse_identifier_result.identifier_string.as_str() {
            "None"
            | "Option::None" => {
                let context: crate::common::rustc::deserializer::Context = parse_identifier_result.finish.into();
                crate::deserializer::Result::NoDataFound {
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
                                    .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                                return deserializer::Result::Err(error);
                            },
                        }
                    },
                    crate::common::rustc::deserializer::Context::ArgumentStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = crate::common::rustc::error::IncompatibleEnumDataType {
                            enum_declaration_found: parse_identifier_result.identifier_string,
                            enum_data_type_expected: crate::common::rustc::error::incompatibleenumdataType::EnumDataType::Argument,
                            context_found: context,
                        };
                        let error = crate::Error::gen(incompatible_enum_data_type)
                            .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                        return deserializer::Result::Err(error);
                    }
                }

                let (argument_0_data, argument_0_context) = match <DataType::Deserializer as crate::deserializer::Trait<DataType, crate::common::rustc::deserializer::Context, crate::common::rustc::error::Source>>::deserialize(reader_mut_ref) {
                    deserializer::Result::DataFound{ data, context} => (data, context),
                    deserializer::Result::NoDataFound { context } => {
                        let unexpected_no_content = crate::common::rustc::error::MissingEnumArgument {
                            enum_declaration_found: parse_identifier_result.identifier_string,
                            argument_type_expected: DataType::definition(),
                        };
                        let error = crate::Error::gen(unexpected_no_content)
                            .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                        return deserializer::Result::Err(error);
                    }
                    deserializer::Result::Err(error) => {
                        let error = error
                            .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                        return deserializer::Result::Err(error);
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
                                    .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                                return deserializer::Result::Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = crate::common::rustc::error::MissingEnumArgumentClosure{
                            enum_declaration_found: parse_identifier_result.identifier_string,
                        };
                        let error = crate::Error::gen(missing_argument_closure)
                            .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                        return deserializer::Result::Err(error);
                    },
                }

                let context = crate::common::rustc::deserializer::Context::Freestanding;
                crate::deserializer::Result::DataFound {
                    data: Some(argument_0_data),
                    context
                }
            },

            _ => {
                let incompatible_declaration_found = crate::common::rustc::error::IncompatibleEnumDeclaration {
                    enum_declaration_found: parse_identifier_result.identifier_string,
                    enum_declaration_expected_entries: error::ExpectedEntries::from(vec![
                        std::string::String::from("Some"),
                        std::string::String::from("Option::Some"),
                        std::string::String::from("None"),
                        std::string::String::from("Option::None"),
                    ]),
                };
                let error = crate::Error::gen(incompatible_declaration_found)
                    .propagate(<Option as crate::deserializer::context::Trait<std::option::Option<DataType>, crate::common::rustc::Format>>::context_general());
                return deserializer::Result::Err(error);
            },
        };

        return result;
    }
}


impl<DataType> crate::Deserialization<crate::common::rustc::Format> for std::option::Option<DataType>
where DataType: crate::Deserialization<crate::common::rustc::Format>
                + crate::identity::Trait
{
    type Deserializer = Option;
}
