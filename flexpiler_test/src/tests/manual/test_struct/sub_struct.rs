

pub struct TestStructSub {
    pub a_usize: usize,
}


struct TestStructSub_flexpiler_Deserializer {
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


impl Into<Result<TestStructSub, flexpiler::Error>> for TestStructSub_flexpiler_Deserializer {
    fn into(self) -> Result<TestStructSub, flexpiler::Error> {
        use flexpiler::error::Trait as ErrorTrait;
        use flexpiler::deserializer::Trait as DeserializerTrait;

        let a_usize = match self.a_usize_option {
            Some(value) => value,
            None => {
                let missing_field = flexpiler::error::MissingStructField {
                    struct_declaration_found: String::from("TestStructSub"),
                    field_declaration_expected: String::from("a_usize"),
                };
                let error = flexpiler::Error::gen(missing_field)
                    .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                return Err(error);
            }
        };

        Ok(TestStructSub {
            a_usize,
        })
    }
}


impl flexpiler::Deserialize<flexpiler::common::rustc::Format> for TestStructSub {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                               -> Result<Self, flexpiler::Error>
        where ReaderType: flexpiler::reader::Trait {
        use flexpiler::Deserialize;
        use flexpiler::deserializer::Trait as DeserializerTrait;
        use flexpiler::error::Trait as ErrorTrait;

        let mut deserializer = TestStructSub_flexpiler_Deserializer::default();

        let struct_declaration_string = match flexpiler::parser::parse::<
            flexpiler::common::rustc::block::IdentifierWithDataStartFinish,
            ReaderType
        > (reader_mut_ref) {
            Err(parser_error) => {
                let error = flexpiler::error::Error::gen(parser_error)
                    .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                return Err(error);
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
                        return Err(error);
                    },
                    Ok(_) => {
                        // successfully parsed data start
                    },
                }

                declaration
            },
        };

        if struct_declaration_string.as_str() != "TestStructSub" {
            let incompatible_declaration_struct = flexpiler::error::IncompatibleStructDeclaration {
                struct_declaration_found: struct_declaration_string,
                struct_declaration_expected: std::string::String::from("TestStructSub"),
            };
            let error = flexpiler::Error::gen(incompatible_declaration_struct)
                .propagate(TestStructSub_flexpiler_Deserializer::context_general());
            return Err(error);
        }

        loop {
            let declaration = match flexpiler::parser::parse::<
                flexpiler::common::rustc::block::DeclarationOrDataEnd,
                ReaderType
            > (reader_mut_ref) {
                Err(parser_error) => {
                    let error = flexpiler::Error::gen(parser_error)
                        .propagate(TestStructSub_flexpiler_Deserializer::context_general());
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
                            let error = error.propagate(TestStructSub_flexpiler_Deserializer::context_field("a_usize"));
                            return Err(error);
                        }
                    }
                }

                _ => {
                    let unrecognized_field_declaration = flexpiler::error::UnrecognizedFieldDeclaration {
                        field_declaration_found: declaration,
                        field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                            String::from("a_usize"),
                        ]),
                    };
                    let error = flexpiler::Error::gen(unrecognized_field_declaration)
                        .propagate(TestStructSub_flexpiler_Deserializer::context_general());
                    return Err(error);
                }
            }
        }

        return deserializer.into();
    }
}

