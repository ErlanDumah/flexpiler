use crate::tests::manual::test_struct::sub_struct::TestStructSub;

pub enum TestEnum {
    NoData,
    DataEmpty(),
    DataSimple(std::string::String),
    DataComplex{
        a_usize: usize,
        a_string: std::string::String,
    },
}


struct TestEnum_flexpiler_Deserializer;


#[derive(Default)]
struct TestEnum_flexpiler_Deserializer_DataComplex {
    a_usize_option: Option<usize>,
    a_string_option: Option<std::string::String>,
}


impl TestEnum_flexpiler_Deserializer {
    fn context_general() -> flexpiler::error::Context {
        let mut trace = std::string::String::from("ManualTestEnum");
        flexpiler::error::Context {
            trace,
        }
    }
    fn context_entry_field(enum_entry_name: &str,
                           field_name: &str) -> flexpiler::error::Context {
        let mut trace = std::string::String::from("ManualTestEnum");
        trace.push_str("::");
        trace.push_str(enum_entry_name);
        trace.push_str("[");
        trace.push_str(field_name);
        trace.push_str("]");
        flexpiler::error::Context {
            trace,
        }
    }
    fn context_entry_general(enum_entry_name: &str) -> flexpiler::error::Context {
        let mut trace = std::string::String::from("ManualTestEnum");
        trace.push_str("::");
        trace.push_str(enum_entry_name);
        flexpiler::error::Context {
            trace,
        }
    }
}

/*
impl Into<Result<ManualTestEnum, flexpiler::Error>> for ManualTestEnum_flexpiler_Deserializer {
    fn into(self) -> Result<ManualTestEnum, Error> {
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::deserializer::Trait as DeserializerTrait;

        return match self {
            ManualTestEnum_flexpiler_Deserializer::NoData => {
                Ok(ManualTestEnum::NoData)
            },
            ManualTestEnum_flexpiler_Deserializer::DataEmpty() => {
                Ok(ManualTestEnum::DataEmpty())
            },
            ManualTestEnum_flexpiler_Deserializer::DataSimple(a_string_option) => {
                let a_string = match a_string_option {
                    Some(value) => value,
                    None => {
                        let missing_field = flexpiler::error::MissingEnumComplexField {
                            struct_declaration_found: String::from("ManualTestEnum::DataSimple"),
                            field_declaration_expected: String::from("a_string"),
                        };
                        let error = flexpiler::Error::gen(missing_field)
                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("DataSimple"));
                        return Err(error);
                    }
                };

                Ok(ManualTestEnum::DataSimple(a_string))
            },
            ManualTestEnum_flexpiler_Deserializer::DataComplex {
                a_usize_option,
                a_string_option,
            } => {
                let a_usize = match a_usize_option {
                    Some(value) => value,
                    None => {
                        let missing_field = flexpiler::error::MissingStructField {
                            field_declaration_expected: String::from("a_usize"),
                        };
                        let error = flexpiler::Error::gen(missing_field)
                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("DataComplex"));
                        return Err(error);
                    }
                };

                let a_string = match a_string_option {
                    Some(value) => value,
                    None => {
                        let missing_field = flexpiler::error::MissingStructField {
                            field_declaration_expected: String::from("a_string"),
                        };
                        let error = flexpiler::Error::gen(missing_field)
                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("DataComplex"));
                        return Err(error);
                    }
                };

                Ok(ManualTestEnum::DataComplex {
                    a_usize,
                    a_string,
                })
            }
        }
    }
}
*/


impl flexpiler::deserializer::Trait<
    TestEnum,
    flexpiler::common::rustc::deserializer::Context
> for TestEnum
    where usize: flexpiler::Deserialization<flexpiler::common::rustc::Format>,
{
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                               -> Result<flexpiler::deserializer::Result<TestEnum, flexpiler::common::rustc::deserializer::Context>, flexpiler::Error>
        where ReaderType: flexpiler::reader::Trait {
        //use flexpiler::Deserialize;
        use flexpiler::Deserialization;
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::parser::Parse;

        let declaration_result = match flexpiler::parser::parse::<
            flexpiler::common::rustc::block::IdentifierWithVariableFinish,
            ReaderType
        > (reader_mut_ref) {
            Err(parser_error) => {
                let error = flexpiler::error::Error::gen(parser_error)
                    .propagate(TestEnum_flexpiler_Deserializer::context_general());
                return Err(error);
            },
            Ok(result) => result,
        };

        let result = match declaration_result.identifier_string.as_str() {
            "ManualTestEnum::NoData" => {
                let context: flexpiler::common::rustc::deserializer::Context = declaration_result.finish.into();
                flexpiler::deserializer::Result {
                    data: TestEnum::NoData,
                    context
                }
            },
            "ManualTestEnum::DataEmpty" => {
                let context: flexpiler::common::rustc::deserializer::Context = declaration_result.finish.into();
                match context {
                    flexpiler::common::rustc::deserializer::Context::Freestanding => {
                        match flexpiler::parser::parse::<
                            flexpiler::common::rustc::block::ArgumentStart,
                            ReaderType
                        >(reader_mut_ref) {
                            Ok(result) => {},
                            Err(parser_error) => {
                                let error = flexpiler::error::Error::gen(parser_error)
                                    .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataEmpty"));
                                return Err(error);
                            },
                        }
                    },
                    flexpiler::common::rustc::deserializer::Context::ArgumentStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = flexpiler::error::IncompatibleEnumDataType {
                            enum_declaration_found: declaration_result.identifier_string,
                            enum_data_type_expected: flexpiler::error::incompatibleenumdataType::EnumDataType::Argument,
                            context_found: context,
                        };
                        let error = flexpiler::Error::gen(incompatible_enum_data_type)
                            .propagate(TestEnum_flexpiler_Deserializer::context_general());
                        return Err(error);
                    }
                }

                match flexpiler::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                    Ok(result) => {},
                    Err(parser_error) => {
                        let error = flexpiler::error::Error::gen(parser_error)
                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataEmpty"));
                        return Err(error);
                    }
                }

                flexpiler::deserializer::Result {
                    data: TestEnum::DataEmpty(),
                    context
                }
            },
            "ManualTestEnum::DataSimple" => {
                let context: flexpiler::common::rustc::deserializer::Context = declaration_result.finish.into();
                match context {
                    flexpiler::common::rustc::deserializer::Context::Freestanding => {
                        match flexpiler::parser::parse::<
                            flexpiler::common::rustc::block::ArgumentStart,
                            ReaderType
                        >(reader_mut_ref) {
                            Ok(result) => {},
                            Err(parser_error) => {
                                let error = flexpiler::error::Error::gen(parser_error)
                                    .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataSimple"));
                                return Err(error);
                            },
                        }
                    },
                    flexpiler::common::rustc::deserializer::Context::ArgumentStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = flexpiler::error::IncompatibleEnumDataType {
                            enum_declaration_found: declaration_result.identifier_string,
                            enum_data_type_expected: flexpiler::error::incompatibleenumdataType::EnumDataType::Argument,
                            context_found: context,
                        };
                        let error = flexpiler::Error::gen(incompatible_enum_data_type)
                            .propagate(TestEnum_flexpiler_Deserializer::context_general());
                        return Err(error);
                    }
                }

                let argument_0_result = match flexpiler::parser::parse::<
                    flexpiler::common::rustc::block::String,
                    ReaderType
                >(reader_mut_ref) {
                    Ok(result) => result,
                    Err(parser_error) => {
                        let error = flexpiler::error::Error::gen(parser_error)
                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataSimple"));
                        return Err(error);
                    },
                };

                let argument_0_context = argument_0_result.finish.into();
                match argument_0_context {
                    flexpiler::common::rustc::deserializer::Context::ArgumentEnd => {},
                    flexpiler::common::rustc::deserializer::Context::Freestanding => {
                        match flexpiler::common::rustc::block::ArgumentEndOrSeparator::parse(reader_mut_ref) {
                            Ok(flexpiler::common::rustc::block::argument_end_or_separator::Result::ArgumentEnd) => {
                            }
                            Ok(flexpiler::common::rustc::block::argument_end_or_separator::Result::Separator) => {
                                match flexpiler::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                                    Ok(_) => {},
                                    Err(parser_error) => {
                                        let error = flexpiler::error::Error::gen(parser_error)
                                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataSimple"));
                                        return Err(error);
                                    }
                                }
                            }
                            Err(parser_error) => {
                                let error = flexpiler::error::Error::gen(parser_error)
                                    .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataSimple"));
                                return Err(error);
                            }
                        }
                    },
                    _ => {
                        let missing_argument_closure = flexpiler::error::MissingEnumArgumentClosure{
                            enum_declaration_found: declaration_result.identifier_string,
                        };
                        let error = flexpiler::Error::gen(missing_argument_closure)
                            .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataSimple"));
                        return Err(error);
                    },
                }

                let context = flexpiler::common::rustc::deserializer::Context::Freestanding;
                flexpiler::deserializer::Result {
                    data: TestEnum::DataSimple(
                        argument_0_result.string,
                    ),
                    context
                }
            },
            "ManualTestEnum::DataComplex" => {
                let context: flexpiler::common::rustc::deserializer::Context = declaration_result.finish.into();
                match context {
                    flexpiler::common::rustc::deserializer::Context::Freestanding => {
                        match flexpiler::parser::parse::<
                            flexpiler::common::rustc::block::DataStart,
                            ReaderType
                        >(reader_mut_ref) {
                            Ok(result) => {},
                            Err(parser_error) => {
                                let error = flexpiler::error::Error::gen(parser_error)
                                    .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataComplex"));
                                return Err(error);
                            },
                        }
                    },
                    flexpiler::common::rustc::deserializer::Context::DataStart => {
                    },
                    _ => {
                        let incompatible_enum_data_type = flexpiler::error::IncompatibleEnumDataType {
                            enum_declaration_found: declaration_result.identifier_string,
                            enum_data_type_expected: flexpiler::error::incompatibleenumdataType::EnumDataType::Complex,
                            context_found: context,
                        };
                        let error = flexpiler::Error::gen(incompatible_enum_data_type)
                            .propagate(TestEnum_flexpiler_Deserializer::context_general());
                        return Err(error);
                    }
                }

                let mut object_context = TestEnum_flexpiler_Deserializer_DataComplex::default();
                loop {
                    let declaration_or_data_end_result = match flexpiler::parser::parse::<
                        flexpiler::common::rustc::block::DeclarationOrDataEnd,
                        ReaderType
                    >(reader_mut_ref) {
                        Ok(declaration_or_data_end_result) => declaration_or_data_end_result,
                        Err(parser_error) => {
                            let error = flexpiler::error::Error::gen(parser_error)
                                .propagate(TestEnum_flexpiler_Deserializer::context_entry_general("ManualTestEnum::DataComplex"));
                            return Err(error);
                        },
                    };

                    let declaration_string = match declaration_or_data_end_result {
                        flexpiler::common::rustc::block::declaration_or_data_end::Result::Declaration(declaration_string) => declaration_string,
                        flexpiler::common::rustc::block::declaration_or_data_end::Result::DataEnd() => {
                            break;
                        },
                    };

                    let mut context = match declaration_string.as_str() {
                        "a_usize" => {
                            let deserializer_result = match <usize as flexpiler::Deserialization<flexpiler::common::rustc::Format>>::Deserializer::deserialize(reader_mut_ref) {
                                Ok(result) => result,
                                Err(deserializer_error) => {
                                    let error = deserializer_error
                                        .propagate(TestEnum_flexpiler_Deserializer::context_entry_field("ManualTestEnum::DataComplex", "a_usize"));
                                    return Err(error);
                                },
                            };

                            object_context.a_usize_option = Some(deserializer_result.data);
                            deserializer_result.context
                        },
                        "a_string" => {
                            let deserializer_result = match <std::string::String as flexpiler::Deserialization<flexpiler::common::rustc::Format>>::Deserializer::deserialize(reader_mut_ref) {
                                Ok(result) => result,
                                Err(deserializer_error) => {
                                    let error = deserializer_error
                                        .propagate(TestEnum_flexpiler_Deserializer::context_entry_field("ManualTestEnum::DataComplex", "a_usize"));
                                    return Err(error);
                                },
                            };

                            object_context.a_string_option = Some(deserializer_result.data);
                            deserializer_result.context
                        },
                        _ => {
                            let unrecognized_field_declaration = flexpiler::error::UnrecognizedFieldDeclaration {
                                field_declaration_found: declaration_string,
                                field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                                    String::from("a_usize"),
                                    String::from("a_string"),
                                ]),
                            };
                            let error = flexpiler::Error::gen(unrecognized_field_declaration)
                                .propagate(TestEnum_flexpiler_Deserializer::context_general());
                            return Err(error);
                        },
                    };

                    if context == flexpiler::common::rustc::deserializer::Context::Freestanding {
                        context = match flexpiler::common::rustc::block::DataEndOrSeparator::parse(reader_mut_ref) {
                            Ok(flexpiler::common::rustc::block::data_end_or_separator::Result::Separator) => flexpiler::common::rustc::deserializer::Context::Separator,
                            Ok(flexpiler::common::rustc::block::data_end_or_separator::Result::DataEnd) => flexpiler::common::rustc::deserializer::Context::DataEnd,
                            Err(parser_error) => {
                                let error = flexpiler::Error::gen(parser_error)
                                    .propagate(TestEnum_flexpiler_Deserializer::context_general());
                                return Err(error);
                            }
                        }
                    }

                    match context {
                        flexpiler::common::rustc::deserializer::Context::DataEnd => break,
                        flexpiler::common::rustc::deserializer::Context::Separator => {},
                        _ => {
                            let unexpected_entry_context_finish = flexpiler::error::UnexpectedEntryFinishContext {
                                entry_declaration: declaration_string,
                                context_found: context,
                                context_expected: flexpiler::error::ExpectedEntries::from(vec![
                                    flexpiler::common::rustc::deserializer::Context::DataEnd,
                                    flexpiler::common::rustc::deserializer::Context::Separator,
                                ]),
                            };
                            let error = flexpiler::Error::gen(unexpected_entry_context_finish)
                                .propagate(TestEnum_flexpiler_Deserializer::context_general());
                            return Err(error);
                        }
                    }
                }

                let data = {
                    let a_usize = match object_context.a_usize_option {
                        Some(value) => value,
                        None => {
                            let missing_field = flexpiler::error::MissingEnumComplexField {
                                enum_declaration_found: String::from("TestEnum::DataComplex"),
                                field_declaration_expected: String::from("a_usize"),
                            };
                            let error = flexpiler::Error::gen(missing_field)
                                .propagate(TestEnum_flexpiler_Deserializer::context_general());
                            return Err(error);
                        }
                    };

                    let a_string = match object_context.a_string_option {
                        Some(value) => value,
                        None => {
                            let missing_field = flexpiler::error::MissingEnumComplexField {
                                enum_declaration_found: String::from("TestEnum::DataComplex"),
                                field_declaration_expected: String::from("a_string"),
                            };
                            let error = flexpiler::Error::gen(missing_field)
                                .propagate(TestEnum_flexpiler_Deserializer::context_general());
                            return Err(error);
                        }
                    };

                    TestEnum::DataComplex {
                        a_usize,
                        a_string,
                    }
                };

                return Ok(flexpiler::deserializer::Result {
                    data,
                    context: flexpiler::common::rustc::deserializer::Context::Freestanding,
                });
            },

            _ => {
                let incompatible_declaration_found = flexpiler::error::IncompatibleEnumDeclaration {
                    enum_declaration_found: declaration_result.identifier_string,
                    enum_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                        std::string::String::from("ManualTestEnum::NoData"),
                        std::string::String::from("ManualTestEnum::DataEmpty"),
                        std::string::String::from("ManualTestEnum::DataSimple"),
                        std::string::String::from("ManualTestEnum::DataComplex"),
                    ]),
                };
                let error = flexpiler::Error::gen(incompatible_declaration_found)
                    .propagate(TestEnum_flexpiler_Deserializer::context_general());
                return Err(error);
            }
        };

        return Ok(result,);

        /*
        let expected_enum_data_type = match declaration_result.identifier_string.as_str() {
            "ManualTestEnum::NoData" => {
                flexpiler::common::rustc::block::identifier_with_variable_finish::EnumDataType::NoData
            },
            "ManualTestEnum::DataEmpty"
            | "ManualTestEnum::DataSimple" => {
                flexpiler::common::rustc::block::identifier_with_variable_finish::EnumDataType::ArgumentData
            },
            "ManualTestEnum::DataComplex" => {
                flexpiler::common::rustc::block::identifier_with_variable_finish::EnumDataType::ComplexData
            },

            _ => {
                let incompatible_declaration_found = flexpiler::error::IncompatibleEnumDeclaration {
                    enum_declaration_found: declaration_result.identifier_string,
                    enum_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                        std::string::String::from("ManualTestEnum::NoData"),
                        std::string::String::from("ManualTestEnum::DataEmpty"),
                        std::string::String::from("ManualTestEnum::DataSimple"),
                        std::string::String::from("ManualTestEnum::DataComplex"),
                    ]),
                };
                let error = flexpiler::Error::gen(incompatible_declaration_found)
                    .propagate(ManualTestEnum_flexpiler_Deserializer::context_general());
                return Err(error);
            },
        };

        if expected_enum_data_type != declaration_result.data_type {
            let incompatible_enum_data_type = flexpiler::error::IncompatibleEnumDataType {
                enum_declaration_found: declaration_result.identifier_string,
                enum_data_type_found: declaration_result.data_type,
                enum_data_type_expected: expected_enum_data_type,
            };
            let error = flexpiler::Error::gen(incompatible_enum_data_type)
                .propagate(ManualTestEnum_flexpiler_Deserializer::context_general());
            return Err(error);
        }

        match declaration_result.identifier_string.as_str() {
            "ManualTestEnum::NoData" => {
                return Ok(ManualTestEnum::NoData//
                );
            },
            "ManualTestEnum::DataEmpty" => {
                let argument_block_result = flexpiler::parser::parse::<
                    flexpiler::common::rustc::block::ar::Result,
                    flexpiler::common::rustc::block::IdentifierWithVariableFinish,
                    ReaderType
                > (reader_mut_ref);

                return Ok(ManualTestEnum::DataEmpty());
            },
            "ManualTestEnum::DataSimple" => {
                flexpiler::common::rustc::block::identifier_with_variable_finish::EnumDataType::ArgumentData

                let argument_0 = match std::string::String::deserialize(reader_mut_ref) {
                    Ok(result) => ;
                }

                return Ok(ManualTestEnum::DataSimple(argument_0));
            },
            "ManualTestEnum::DataComplex" => {
                loop {

                }

                return Ok(ManualTestEnum::DataComplex {
                    a_usize,
                    a_string,
                });
            },
        }
         */
    }
}

