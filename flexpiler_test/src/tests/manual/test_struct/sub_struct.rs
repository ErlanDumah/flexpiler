use flexpiler::common::rustc::Format;

pub struct TestStructSub {
    pub a_usize: usize,
}


impl flexpiler::identity::Trait for TestStructSub {
    fn definition() -> std::string::String {
        return std::string::String::from("TestStructSub");
    }
}


pub struct TestStructSub_flexpiler_Deserializer {
    pub a_usize_option: Option<usize>,
}


impl Default for TestStructSub_flexpiler_Deserializer {
    fn default() -> Self {
        TestStructSub_flexpiler_Deserializer {
            a_usize_option: None,
        }
    }
}


impl TestStructSub_flexpiler_Deserializer {
    fn context_field(field_name: &str) -> flexpiler::error::Context {
        let mut trace = std::string::String::from("TestStructSub");
        trace.push_str("[");
        trace.push_str(field_name);
        trace.push_str("]");
        flexpiler::error::Context {
            trace,
        }
    }
    fn context_general() -> flexpiler::error::Context {
        flexpiler::error::Context {
            trace: String::from("TestStructSub"),
        }
    }
}


impl Into<Result<TestStructSub, flexpiler::Error<flexpiler::common::rustc::error::Source>>> for TestStructSub_flexpiler_Deserializer {
    fn into(self) -> Result<TestStructSub, flexpiler::Error<flexpiler::common::rustc::error::Source>> {
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::error::propagation::Trait as ErrorPropagationTrait;
        use flexpiler::deserializer::Trait as DeserializerTrait;

        let a_usize = match self.a_usize_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::common::rustc::error::MissingStructField {
                    struct_declaration_found: String::from("TestStructSub"),
                    field_declaration_expected: String::from("a_usize"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                return Err(error);
            }
        };

        Ok(TestStructSub {
            a_usize
        })
    }
}


impl flexpiler::deserializer::Trait<
    TestStructSub,
    flexpiler::common::rustc::Format
> for TestStructSub_flexpiler_Deserializer {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                               -> flexpiler::deserializer::Result<TestStructSub, flexpiler::common::rustc::deserializer::Context, flexpiler::Error<flexpiler::common::rustc::error::Source>>
        where ReaderType: flexpiler::reader::Trait {
        use flexpiler::Deserialize;
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::error::propagation::Trait as ErrorPropagationTrait;
        use flexpiler::identity::Trait;
        use flexpiler::parser::Parse;

        let mut deserializer = TestStructSub_flexpiler_Deserializer::default();

        let struct_declaration_string = match flexpiler::parser::parse::<
            flexpiler::common::rustc::block::IdentifierWithDataStartFinish,
            ReaderType
        > (reader_mut_ref) {
            Err(parser_error) => {
                let error = flexpiler::Error::gen(parser_error)
                    .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                return flexpiler::deserializer::Result::Err(error);
            },
            Ok(flexpiler::common::rustc::block::identifier_with_data_start_finish::Result::DataStartFinish(declaration)) => {
                declaration
            },
            Ok(flexpiler::common::rustc::block::identifier_with_data_start_finish::Result::Freestanding(declaration)) => {
                match flexpiler::parser::parse::<
                    flexpiler::common::rustc::block::DataStart,
                    ReaderType
                > (reader_mut_ref) {
                    Err(parser_error) => {
                        let error = flexpiler::Error::gen(parser_error)
                            .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                        return flexpiler::deserializer::Result::Err(error);
                    },
                    Ok(_) => {
                        // successfully parsed data start
                    },
                }

                declaration
            },
        };

        if struct_declaration_string.as_str() != "TestStructSub" {
            let incompatible_declaration_struct = flexpiler::common::rustc::error::IncompatibleStructDeclaration {
                struct_declaration_found: struct_declaration_string,
                struct_declaration_expected: std::string::String::from("TestStructSub"),
            };
            let error = flexpiler::Error::gen(incompatible_declaration_struct)
                .propagate(TestStructSub_flexpiler_Deserializer::context_general());
            return flexpiler::deserializer::Result::Err(error);
        }

        loop {
            let declaration = match flexpiler::parser::parse::<
                flexpiler::common::rustc::block::DeclarationOrDataEnd,
                ReaderType
            > (reader_mut_ref) {
                Err(parser_error) => {
                    let error = flexpiler::Error::gen(parser_error)
                        .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                    return flexpiler::deserializer::Result::Err(error);
                },
                Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::DataEnd()) => {
                    break;
                },
                Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::Declaration(declaration)) => {
                    declaration
                },
            };

            let mut context = match declaration.as_str() {
                "a_usize" => {
                    let result = flexpiler::common::rustc::deserializer::PrimitiveUSize::deserialize(reader_mut_ref);
                    match result {
                        flexpiler::deserializer::Result::DataFound {data, context} => {
                            deserializer.a_usize_option = Some(data);
                            context
                        },
                        flexpiler::deserializer::Result::NoDataFound { context } => {
                            let unexpected_no_content = flexpiler::error::source::common::UnexpectedNoContent {
                                definition_expected: <usize as flexpiler::identity::Trait>::definition(),
                            };
                            let error_source_common: flexpiler::error::source::Common = unexpected_no_content.into();
                            let error = flexpiler::Error::gen(error_source_common)
                                .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                            return flexpiler::deserializer::Result::Err(error);
                        },
                        flexpiler::deserializer::Result::Err(error) => {
                            let error = error.propagate(TestStructSub_flexpiler_Deserializer::context_field("a_usize"));
                            return flexpiler::deserializer::Result::Err(error);
                        }
                    }
                }

                _ => {
                    let unrecognized_field_declaration = flexpiler::common::rustc::error::UnrecognizedFieldDeclaration {
                        field_declaration_found: declaration,
                        field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                            String::from("a_usize"),
                        ]),
                    };
                    let error = flexpiler::Error::gen(unrecognized_field_declaration)
                        .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                    return flexpiler::deserializer::Result::Err(error);
                }
            };

            if context == flexpiler::common::rustc::deserializer::Context::Freestanding {
                match flexpiler::common::rustc::block::ContextDenominator::parse(reader_mut_ref) {
                    Ok(result) => {
                        context = result.finish.into();
                    },
                    Err(parser_error) => {
                        let error = flexpiler::Error::gen(parser_error)
                            .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                        return flexpiler::deserializer::Result::Err(error);
                    }
                }
            }

            match context {
                flexpiler::common::rustc::deserializer::Context::DataEnd => {
                    break;
                },
                flexpiler::common::rustc::deserializer::Context::Separator => {
                    //do nothing
                },
                _ => {
                    let unexpected_entry_finish = flexpiler::common::rustc::error::UnexpectedEntryFinishContext {
                        entry_declaration: declaration,
                        context_found: context,
                        context_expected: flexpiler::error::ExpectedEntries::from(vec![
                            flexpiler::common::rustc::deserializer::Context::DataEnd,
                            flexpiler::common::rustc::deserializer::Context::Separator,
                        ])
                    };
                    let error = flexpiler::Error::gen(unexpected_entry_finish)
                        .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                    return flexpiler::deserializer::Result::Err(error);
                }
            }
        }

        return match deserializer.into() {
            Ok(data) => {
                flexpiler::deserializer::Result::DataFound {
                    data,
                    context: flexpiler::common::rustc::deserializer::Context::Freestanding,
                }
            },
            Err(error) => {
                flexpiler::deserializer::Result::Err(error)
            }
        };
    }
}


impl flexpiler::Deserialization<flexpiler::common::rustc::Format> for TestStructSub {
    type Deserializer = TestStructSub_flexpiler_Deserializer;
}


#[test]
fn basic_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestStructSub{ a_usize: 60 }"
    );

    let parse_result = TestStructSub::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_usize, 60,
               "simple_manual_teststruct_manualsubteststruct_basic_serialisation_successful(): a_usize deserialised value had unexpected usize value {}, expected {}",
               test_struct.a_usize,
               60);
}

