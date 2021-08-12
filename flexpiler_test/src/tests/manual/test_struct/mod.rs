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


struct TestStructFlexpilerDeserializer {
    pub a_i32_option: Option<i32>,
    pub a_usize_option: Option<usize>,
    pub a_string_option: Option<String>,
    pub a_sub_option: Option<TestStructSub>,
}


impl TestStructFlexpilerDeserializer {
    fn context_field(field_name: &str) -> flexpiler::error::Context {
        flexpiler::error::Context {
            trace: String::from(format!("TestStruct[{}]", field_name)),
        }
    }
    fn context_general() -> flexpiler::error::Context {
        flexpiler::error::Context {
            trace: String::from("TestStruct"),
        }
    }
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


impl Into<Result<TestStruct, flexpiler::Error>> for TestStructFlexpilerDeserializer {
    fn into(self) -> Result<TestStruct, flexpiler::Error> {
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::error::Trait as ErrorTrait;

        let a_i32 = match self.a_i32_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_i32"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(TestStructFlexpilerDeserializer::context_general());
                return Err(error);
            }
        };

        let a_usize = match self.a_usize_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_usize"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(TestStructFlexpilerDeserializer::context_general());
                return Err(error);
            }
        };

        let a_string = match self.a_string_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_string"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(TestStructFlexpilerDeserializer::context_general());
                return Err(error);
            }
        };

        let a_sub = match self.a_sub_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::error::MissingStructField {
                    struct_declaration_found: String::from("ManualSubTestStruct"),
                    field_declaration_expected: String::from("a_sub"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(TestStructFlexpilerDeserializer::context_general());
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


impl flexpiler::Deserialize<flexpiler::common::rustc::Format> for TestStruct {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                               -> Result<Self, flexpiler::Error>
        where ReaderType: flexpiler::reader::Trait {
        use flexpiler::Deserialize;
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::error::Trait as ErrorTrait;

        let mut deserializer = TestStructFlexpilerDeserializer::default();

        let struct_declaration_string = match flexpiler::parser::parse::<
            flexpiler::common::rustc::block::IdentifierWithDataStartFinish,
            ReaderType
        > (reader_mut_ref) {
            Err(parser_error) => {
                let error = flexpiler::Error::gen(parser_error)
                    .propagate(TestStructFlexpilerDeserializer::context_general());
                return Err(error);
            },
            Ok(flexpiler::common::rustc::block::identifier_with_data_start_finish::Result::DataStartFinish(declaration)) => {
                declaration
            },
            Ok(flexpiler::common::rustc::block::identifier_with_data_start_finish::Result::Freestanding(declaration)) => {
                match flexpiler::parser::parse::<
                    flexpiler::common::rustc::block::data_start::DataStart,
                    ReaderType
                > (reader_mut_ref) {
                    Err(parser_error) => {
                        let error = flexpiler::Error::gen(parser_error)
                            .propagate(TestStructFlexpilerDeserializer::context_general());
                        return Err(error);
                    },
                    Ok(_) => {
                        // successfully parsed data start
                    },
                }

                declaration
            },
        };

        if struct_declaration_string.as_str() != "TestStruct" {
            let incompatible_struct_declaration = flexpiler::error::IncompatibleStructDeclaration {
                struct_declaration_expected: String::from("TestStruct"),
                struct_declaration_found: struct_declaration_string,
            };
            let error = flexpiler::Error::gen(incompatible_struct_declaration)
                .propagate(TestStructFlexpilerDeserializer::context_general());
            return Err(error);
        }

        loop {
            let declaration = match flexpiler::parser::parse::<
                flexpiler::common::rustc::block::DeclarationOrDataEnd,
                ReaderType
            > (reader_mut_ref) {
                Err(parser_error) => {
                    let error = flexpiler::error::Error::gen(parser_error)
                        .propagate(TestStructFlexpilerDeserializer::context_general());
                    return Err(error);
                },
                Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::DataEnd()) => {
                    break;
                },
                Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::Declaration(declaration)) => {
                    declaration
                },
            };

            match declaration.as_str() {
                "a_usize" => {
                    let result = flexpiler::common::rustc::deserializer::PrimitiveUSize::deserialize(reader_mut_ref);
                    match result {
                        Ok(value) => {
                            deserializer.a_usize_option = Some(value.data);
                        }
                        Err(error) => {
                            let error = error.propagate(TestStructFlexpilerDeserializer::context_field("a_usize"));
                            return Err(error);
                        }
                    }
                },

                "a_string" => {
                    let result = std::string::String::deserialize(reader_mut_ref);
                    match result {
                        Ok(value) => {
                            deserializer.a_string_option = Some(value);
                        }
                        Err(error) => {
                            let error = error.propagate(TestStructFlexpilerDeserializer::context_field("a_string"));
                            return Err(error);
                        }
                    }
                },

                "a_i32" => {
                    let result = flexpiler::common::rustc::deserializer::PrimitiveI32::deserialize(reader_mut_ref);
                    match result {
                        Ok(value) => {
                            deserializer.a_i32_option = Some(value.data);
                        }
                        Err(error) => {
                            let error = error.propagate(TestStructFlexpilerDeserializer::context_field("a_i32"));
                            return Err(error);
                        }
                    }
                },

                "a_sub" => {
                    let result = TestStructSub::deserialize(reader_mut_ref);
                    match result {
                        Ok(value) => {
                            deserializer.a_sub_option = Some(value);
                        }
                        Err(error) => {
                            let error = error.propagate(TestStructFlexpilerDeserializer::context_field("a_sub"));
                            return Err(error);
                        }
                    }
                },

                _ => {
                    let unrecognized_field = flexpiler::error::UnrecognizedFieldDeclaration {
                        field_declaration_found: declaration,
                        field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                            String::from("a_i32"),
                            String::from("a_usize"),
                            String::from("a_string"),
                            String::from("a_sub"),
                        ]),
                    };
                    let error = flexpiler::Error::gen(unrecognized_field)
                        .propagate(TestStructFlexpilerDeserializer::context_general());
                    return Err(error);
                }
            }
        }

        return deserializer.into();
    }
}

