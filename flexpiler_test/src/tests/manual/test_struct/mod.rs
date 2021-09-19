pub mod sub_struct;


use sub_struct::TestStructSub;
use flexpiler::common::rustc::Format;
use flexpiler::Error;


pub struct TestStruct {
    pub a_i32: i32,
    pub a_usize: usize,
    pub a_string: String,
    pub a_sub: TestStructSub,
}


impl flexpiler::identity::Trait for TestStruct {
    fn definition() -> String {
        return std::string::String::from("TestStruct");
    }
}


pub struct TestStructFlexpilerDeserializer {
    pub a_i32_option: Option<i32>,
    pub a_usize_option: Option<usize>,
    pub a_string_option: Option<String>,
    pub a_sub_option: Option<TestStructSub>,
}


impl Default for TestStructFlexpilerDeserializer {
    fn default() -> Self {
        TestStructFlexpilerDeserializer {
            a_i32_option: None,
            a_usize_option: None,
            a_string_option: None,
            a_sub_option: None,
        }
    }
}


impl std::convert::TryInto<TestStruct> for TestStructFlexpilerDeserializer {
    type Error = flexpiler::Error<flexpiler::common::rustc::error::Source>;

    fn try_into(self) -> Result<TestStruct, Self::Error> {
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::deserializer::context::Trait as DeserializerContextTrait;
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::error::propagation::Trait as ErrorPropagationTrait;

        let a_i32 = match self.a_i32_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::common::rustc::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_i32"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                return Err(error);
            }
        };

        let a_usize = match self.a_usize_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::common::rustc::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_usize"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                return Err(error);
            }
        };

        let a_string = match self.a_string_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::common::rustc::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_string"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                return Err(error);
            }
        };

        let a_sub = match self.a_sub_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::common::rustc::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_sub"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                return Err(error);
            }
        };

        Ok(TestStruct {
            a_i32,
            a_usize,
            a_string,
            a_sub,
        })
    }
}


impl flexpiler::deserializer::Trait<
    TestStruct,
    flexpiler::common::rustc::Format,
> for TestStructFlexpilerDeserializer {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                               -> flexpiler::deserializer::Result<
                                   TestStruct,
                                   flexpiler::common::rustc::deserializer::Context,
                                   flexpiler::Error<flexpiler::common::rustc::error::Source>
                               >
    where ReaderType: flexpiler::reader::Trait {
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::deserializer::context::Trait as DeserializerContextTrait;
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::error::propagation::Trait as ErrorPropagationTrait;
        use flexpiler::identity::Trait;
        use flexpiler::parser::Parse;

        let mut deserializer = TestStructFlexpilerDeserializer::default();

        let (identifier_data, identifier_finish) = match flexpiler::common::rustc::block::Identifier::parse(reader_mut_ref) {
            Ok(flexpiler::common::rustc::block::identifier::Result::NoDataFound { finish }) => {
                return flexpiler::deserializer::Result::NoDataFound {
                    context: finish.into()
                };
            },
            Ok(flexpiler::common::rustc::block::identifier::Result::DataFound { data, finish }) => {
                (data, finish)
            },
            Err(parser_error) => {
                let error = flexpiler::Error::gen(parser_error)
                    .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                return flexpiler::deserializer::Result::Err(error);
            }
        };
        let mut context: flexpiler::common::rustc::deserializer::Context = identifier_finish.into();
        if context == flexpiler::common::rustc::deserializer::Context::Freestanding {
            context = match flexpiler::common::rustc::block::ContextDenominator::parse(reader_mut_ref) {
                Ok(result) => {
                    result.finish.into()
                },
                Err(parser_error) => {
                    let error = flexpiler::Error::gen(parser_error)
                        .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                    return flexpiler::deserializer::Result::Err(error);
                },
            }
        }
        match context {
            flexpiler::common::rustc::deserializer::Context::DataStart => {},
            _ => {
                let unexpected_context = flexpiler::common::rustc::error::UnexpectedContext {
                    context_found: context,
                    context_expected: flexpiler::error::ExpectedEntries::from(vec![
                        flexpiler::common::rustc::deserializer::Context::DataStart,
                    ]),
                };
                let error = flexpiler::Error::gen(unexpected_context)
                    .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::Trait<TestStruct, flexpiler::common::rustc::Format>>::context());
                return flexpiler::deserializer::Result::Err(error);
            },
        }

        if identifier_data.as_str() != "TestStruct" {
            let incompatible_struct_declaration = flexpiler::common::rustc::error::IncompatibleStructDeclaration {
                struct_declaration_expected: String::from("TestStruct"),
                struct_declaration_found: identifier_data,
            };
            let error = flexpiler::Error::gen(incompatible_struct_declaration)
                .propagate(TestStructFlexpilerDeserializer::context());
            return flexpiler::deserializer::Result::Err(error);
        }

        loop {
            let declaration = match flexpiler::parser::parse::<
                flexpiler::common::rustc::block::DeclarationOrDataEnd,
                ReaderType
            > (reader_mut_ref) {
                Err(parser_error) => {
                    let error = flexpiler::error::Error::gen(parser_error)
                        .propagate(TestStructFlexpilerDeserializer::context());
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
                                .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_usize"));
                            return flexpiler::deserializer::Result::Err(error);
                        },
                        flexpiler::deserializer::Result::Err(error) => {
                            let error = error.propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_usize"));
                            return flexpiler::deserializer::Result::Err(error);
                        }
                    }
                },

                "a_string" => {
                    match <std::string::String as flexpiler::Deserialization<flexpiler::common::rustc::Format>>::Deserializer::deserialize(reader_mut_ref) {
                        flexpiler::deserializer::Result::DataFound { data, context } => {
                            deserializer.a_string_option = Some(data);
                            context
                        },
                        flexpiler::deserializer::Result::NoDataFound { context } => {
                            let unexpected_no_content = flexpiler::error::source::common::UnexpectedNoContent {
                                definition_expected: <std::string::String as flexpiler::identity::Trait>::definition(),
                            };
                            let error_source_common: flexpiler::error::source::Common = unexpected_no_content.into();
                            let error = flexpiler::Error::gen(error_source_common)
                                .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_string"));
                            return flexpiler::deserializer::Result::Err(error);
                        }
                        flexpiler::deserializer::Result::Err(error) => {
                            let error = error
                                .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_string"));
                            return flexpiler::deserializer::Result::Err(error);
                        }
                    }
                },

                "a_i32" => {
                    let result = flexpiler::common::rustc::deserializer::PrimitiveI32::deserialize(reader_mut_ref);
                    match result {
                        flexpiler::deserializer::Result::DataFound{ data, context } => {
                            deserializer.a_i32_option = Some(data);
                            context
                        }
                        flexpiler::deserializer::Result::NoDataFound { context } => {
                            let unexpected_no_content = flexpiler::error::source::common::UnexpectedNoContent {
                                definition_expected: <i32 as flexpiler::identity::Trait>::definition(),
                            };
                            let error_source_common: flexpiler::error::source::Common = unexpected_no_content.into();
                            let error = flexpiler::Error::gen(error_source_common)
                                .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_i32"));
                            return flexpiler::deserializer::Result::Err(error);
                        },
                        flexpiler::deserializer::Result::Err(error) => {
                            let error = error.propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_i32"));
                            return flexpiler::deserializer::Result::Err(error);
                        }
                    }
                },

                "a_sub" => {
                    let result = <TestStructSub as flexpiler::Deserialization<flexpiler::common::rustc::Format>>::Deserializer::deserialize(reader_mut_ref);
                    match result {
                        flexpiler::deserializer::Result::DataFound{ data, context } => {
                            deserializer.a_sub_option = Some(data);
                            context
                        },
                        flexpiler::deserializer::Result::NoDataFound{ context } => {
                            let unexpected_no_content = flexpiler::error::source::common::UnexpectedNoContent {
                                definition_expected: <TestStructSub as flexpiler::identity::Trait>::definition(),
                            };
                            let error_source_common: flexpiler::error::source::Common = unexpected_no_content.into();
                            let error = flexpiler::Error::gen(error_source_common)
                                .propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_sub"));
                            return flexpiler::deserializer::Result::Err(error);
                        },
                        flexpiler::deserializer::Result::Err(error) => {
                            let error = error.propagate(<TestStructFlexpilerDeserializer as flexpiler::deserializer::context::FieldTrait<TestStruct, flexpiler::common::rustc::Format>>::context_field("a_sub"));
                            return flexpiler::deserializer::Result::Err(error);
                        },
                    }
                },

                _ => {
                    let unrecognized_field = flexpiler::common::rustc::error::UnrecognizedFieldDeclaration {
                        field_declaration_found: declaration,
                        field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                            String::from("a_i32"),
                            String::from("a_usize"),
                            String::from("a_string"),
                            String::from("a_sub"),
                        ]),
                    };
                    let error = flexpiler::Error::gen(unrecognized_field)
                        .propagate(TestStructFlexpilerDeserializer::context());
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
                            .propagate(TestStructFlexpilerDeserializer::context());
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
                        .propagate(TestStructFlexpilerDeserializer::context());
                    return flexpiler::deserializer::Result::Err(error);
                }
            }
        }

        return match <TestStructFlexpilerDeserializer as std::convert::TryInto<TestStruct>>::try_into(deserializer) {
            Ok(data) => {
                flexpiler::deserializer::Result::DataFound {
                    data,
                    context: flexpiler::common::rustc::deserializer::Context::Freestanding,
                }
            },
            Err(error) => {
                flexpiler::deserializer::Result::Err(error)
            }
        }
    }
}


impl flexpiler::Deserialization<flexpiler::common::rustc::Format> for TestStruct {
    type Deserializer = TestStructFlexpilerDeserializer;
}


#[test]
fn basic_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestStruct{ a_sub: TestStructSub{ a_usize: 60, }, a_string: \"Hello\", a_i32: -34, a_usize: 50 }"
    );

    let parse_result = TestStruct::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_manual_test_struct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_string, "Hello",
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_string,
               "Hello");

    assert_eq!(test_struct.a_usize, 50,
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_usize,
               50);

    assert_eq!(test_struct.a_i32, -34,
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               -34);

    assert_eq!(test_struct.a_sub.a_usize, 60,
               "simple_manual_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_sub.a_usize,
               60);
}

