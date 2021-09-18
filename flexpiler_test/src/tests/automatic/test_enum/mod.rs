mod subclass;


#[derive(flexpiler::Deserialize)]
pub enum TestEnum {
    NoData,
    DataEmpty(),
    DataSimple(std::string::String),
    DataComplex{
        a_usize: usize,
        a_string: std::string::String,
    },
}


/*








impl flexpiler :: identity :: Trait for TestEnum
{
    fn definition() -> String
    { return std :: string :: String :: from("TestEnum") ; }
} pub struct TestEnumflexpilerDeserializer ; pub struct
TestEnumDataComplexflexpilerContext
{
    a_usize_option : Option < usize >, a_string_option : Option < std ::
    string :: String >,
} impl Default for TestEnumDataComplexflexpilerContext
{
    fn default() -> Self
    {
        TestEnumDataComplexflexpilerContext
        { a_usize_option : None, a_string_option : None, }
    }
} impl std :: convert :: TryInto < TestEnum > for
TestEnumDataComplexflexpilerContext
{
    type Error = flexpiler :: Error < flexpiler :: common :: rustc :: error ::
    Source > ; fn try_into(self) -> std :: result :: Result < TestEnum, Self
    :: Error >
    {
        use flexpiler :: deserializer :: context :: Trait as
        DeserializerContextTrait ; use flexpiler :: deserializer :: context ::
        VariantTrait as DeserializerVariantTrait ; use flexpiler :: error ::
        Trait as ErrorTrait ; use flexpiler :: error :: propagation :: Trait
        as ErrorPropagationTrait ; let a_usize = match self . a_usize_option
        {
            Some(value) => value, None =>
            {
                let missing_enum_field = flexpiler :: common :: rustc :: error
                :: MissingEnumComplexField
                {
                    enum_declaration_found : std :: string :: String ::
                    from("TestEnum::DataComplex"), field_declaration_expected
                    : std :: string :: String :: from("a_usize"),
                } ; let error = flexpiler :: Error :: gen(missing_enum_field)
                .
                propagate(< TestEnumflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: VariantTrait < TestEnum,
                          flexpiler :: common :: rustc :: Format >> ::
                          context_variant("DataComplex")) ; return std ::
                result :: Result :: Err(error) ;
            }
        } ; let a_string = match self . a_string_option
        {
            Some(value) => value, None =>
            {
                let missing_enum_field = flexpiler :: common :: rustc :: error
                :: MissingEnumComplexField
                {
                    enum_declaration_found : std :: string :: String ::
                    from("TestEnum::DataComplex"), field_declaration_expected
                    : std :: string :: String :: from("a_string"),
                } ; let error = flexpiler :: Error :: gen(missing_enum_field)
                .
                propagate(< TestEnumflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: VariantTrait < TestEnum,
                          flexpiler :: common :: rustc :: Format >> ::
                          context_variant("DataComplex")) ; return std ::
                result :: Result :: Err(error) ;
            }
        } ; Ok(TestEnum :: DataComplex { a_usize, a_string, })
    }
} impl flexpiler :: deserializer :: Trait < TestEnum, flexpiler :: common ::
rustc :: Format > for TestEnumflexpilerDeserializer where std :: string ::
String : flexpiler :: Deserialization < flexpiler :: common :: rustc :: Format
>, usize : flexpiler :: Deserialization < flexpiler :: common :: rustc ::
Format >, std :: string :: String : flexpiler :: Deserialization < flexpiler
:: common :: rustc :: Format >,
{
    fn deserialize < ReaderType > (reader_mut_ref : & mut ReaderType) ->
    flexpiler :: deserializer :: Result < TestEnum, flexpiler :: common ::
    rustc :: deserializer :: Context, flexpiler :: Error < flexpiler :: common
    :: rustc :: error :: Source >> where ReaderType : flexpiler :: reader ::
    Trait
    {
        use std :: convert :: TryInto ; use flexpiler :: Deserialization ; use
        flexpiler :: deserializer :: Trait as DeserializerTrait ; use
        flexpiler :: deserializer :: context :: VariantTrait as
        DeserializerContextVariantTrait ; use flexpiler :: error :: Trait as
        ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
        ErrorPropagationTrait ; use flexpiler :: parser :: Parse ; let
        declaration_result = match flexpiler :: common :: rustc :: block ::
        IdentifierWithVariableFinish :: parse(reader_mut_ref)
        {
            Err(parser_error) =>
            {
                let error = flexpiler :: Error :: gen(parser_error) .
                propagate(< TestEnumflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: Trait < TestEnum,
                          flexpiler :: common :: rustc :: Format >> ::
                          context()) ; return flexpiler :: deserializer ::
                Result :: Err(error) ;
            }, Ok(result) => result,
        } ; let result = match declaration_result . identifier_string .
        as_str()
        {
            "TestEnum::NoData" =>
            {
                let context : flexpiler :: common :: rustc :: deserializer ::
                Context = declaration_result . finish . into() ; flexpiler ::
                deserializer :: Result :: DataFound
                { data : TestEnum :: NoData, context }
            } "TestEnum::DataEmpty" =>
            {
                let context : flexpiler :: common :: rustc :: deserializer ::
                Context = declaration_result . finish . into() ; match context
                {
                    flexpiler :: common :: rustc :: deserializer :: Context ::
                    Freestanding =>
                    {
                        match flexpiler :: parser :: parse :: < flexpiler ::
                        common :: rustc :: block :: ArgumentStart, ReaderType
                        > (reader_mut_ref)
                        {
                            Ok(result) => { }, Err(parser_error) =>
                            {
                                let error = flexpiler :: Error ::
                                gen(parser_error) .
                                propagate(< TestEnumflexpilerDeserializer as
                                          flexpiler :: deserializer :: context
                                          :: VariantTrait < TestEnum,
                                          flexpiler :: common :: rustc ::
                                          Format >> ::
                                          context_variant("DataEmpty")) ;
                                return flexpiler :: deserializer :: Result ::
                                Err(error) ;
                            },
                        }
                    }, flexpiler :: common :: rustc :: deserializer :: Context
                    :: ArgumentStart => { }, _ =>
                    {
                        let incompatible_enum_data_type = flexpiler :: common
                        :: rustc :: error :: IncompatibleEnumDataType
                        {
                            enum_declaration_found : declaration_result .
                            identifier_string, enum_data_type_expected :
                            flexpiler :: common :: rustc :: error ::
                            incompatibleenumdataType :: EnumDataType ::
                            Argument, context_found : context,
                        } ; let error = flexpiler :: Error ::
                        gen(incompatible_enum_data_type) .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> :: context_variant("DataEmpty")) ;
                        return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    }
                } match flexpiler :: common :: rustc :: block :: ArgumentEnd
                :: parse(reader_mut_ref)
                {
                    Ok(result) => { }, Err(parser_error) =>
                    {
                        let error = flexpiler :: error :: Error ::
                        gen(parser_error) .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> ::
                                  context_variant("TestEnum::DataEmpty")) ;
                        return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    }
                } let data = TestEnum :: DataEmpty() ; flexpiler ::
                deserializer :: Result :: DataFound
                {
                    context : flexpiler :: common :: rustc :: deserializer ::
                    Context :: Freestanding, data,
                }
            } "TestEnum::DataSimple" =>
            {
                let context : flexpiler :: common :: rustc :: deserializer ::
                Context = declaration_result . finish . into() ; match context
                {
                    flexpiler :: common :: rustc :: deserializer :: Context ::
                    Freestanding =>
                    {
                        match flexpiler :: parser :: parse :: < flexpiler ::
                        common :: rustc :: block :: ArgumentStart, ReaderType
                        > (reader_mut_ref)
                        {
                            Ok(result) => { }, Err(parser_error) =>
                            {
                                let error = flexpiler :: Error ::
                                gen(parser_error) .
                                propagate(< TestEnumflexpilerDeserializer as
                                          flexpiler :: deserializer :: context
                                          :: VariantTrait < TestEnum,
                                          flexpiler :: common :: rustc ::
                                          Format >> ::
                                          context_variant("DataSimple")) ;
                                return flexpiler :: deserializer :: Result ::
                                Err(error) ;
                            },
                        }
                    }, flexpiler :: common :: rustc :: deserializer :: Context
                    :: ArgumentStart => { }, _ =>
                    {
                        let incompatible_enum_data_type = flexpiler :: common
                        :: rustc :: error :: IncompatibleEnumDataType
                        {
                            enum_declaration_found : declaration_result .
                            identifier_string, enum_data_type_expected :
                            flexpiler :: common :: rustc :: error ::
                            incompatibleenumdataType :: EnumDataType ::
                            Argument, context_found : context,
                        } ; let error = flexpiler :: Error ::
                        gen(incompatible_enum_data_type) .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> :: context_variant("DataSimple"))
                        ; return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    }
                } let(argument_0_data, argument_0_context) = match < std ::
                string :: String as flexpiler :: Deserialization < flexpiler
                :: common :: rustc :: Format >> :: Deserializer ::
                deserialize(reader_mut_ref)
                {
                    flexpiler :: deserializer :: Result :: DataFound
                    { data, context } => (data, context), flexpiler ::
                    deserializer :: Result :: NoDataFound { context } =>
                    {
                        let missing_enum_argument = flexpiler :: common ::
                        rustc :: error :: MissingEnumArgument
                        {
                            enum_declaration_found : declaration_result .
                            identifier_string, argument_type_expected : < std
                            :: string :: String as flexpiler :: identity ::
                            Trait > :: definition(),
                        } ; let error = flexpiler :: Error ::
                        gen(missing_enum_argument) .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> :: context_variant("DataSimple"))
                        ; return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    }, flexpiler :: deserializer :: Result :: Err(error) =>
                    {
                        let error = error .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> :: context_variant("DataSimple"))
                        ; return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    }
                } ; match argument_0_context
                {
                    flexpiler :: common :: rustc :: deserializer :: Context ::
                    ArgumentEnd => { }, flexpiler :: common :: rustc ::
                    deserializer :: Context :: Separator =>
                    {
                        match flexpiler :: common :: rustc :: block ::
                        ArgumentEnd :: parse(reader_mut_ref)
                        {
                            Ok(_) => { }, Err(parser_error) =>
                            {
                                let error = flexpiler :: error :: Error ::
                                gen(parser_error) .
                                propagate(< TestEnumflexpilerDeserializer as
                                          flexpiler :: deserializer :: context
                                          :: VariantTrait < TestEnum,
                                          flexpiler :: common :: rustc ::
                                          Format >> ::
                                          context_variant("TestEnum::DataSimple"))
                                ; return flexpiler :: deserializer :: Result
                                :: Err(error) ;
                            },
                        }
                    }, flexpiler :: common :: rustc :: deserializer :: Context
                    :: Freestanding =>
                    {
                        match flexpiler :: common :: rustc :: block ::
                        ArgumentEndOrSeparator :: parse(reader_mut_ref)
                        {
                            Ok(flexpiler :: common :: rustc :: block ::
                               argument_end_or_separator :: Result ::
                               ArgumentEnd) => { },
                            Ok(flexpiler :: common :: rustc :: block ::
                               argument_end_or_separator :: Result ::
                               Separator) =>
                            {
                                match flexpiler :: common :: rustc :: block ::
                                ArgumentEnd :: parse(reader_mut_ref)
                                {
                                    Ok(_) => { }, Err(parser_error) =>
                                    {
                                        let error = flexpiler :: error ::
                                        Error :: gen(parser_error) .
                                        propagate(<
                                                  TestEnumflexpilerDeserializer
                                                  as flexpiler :: deserializer
                                                  :: context :: VariantTrait <
                                                  TestEnum, flexpiler ::
                                                  common :: rustc :: Format >>
                                                  ::
                                                  context_variant("TestEnum::DataSimple"))
                                        ; return flexpiler :: deserializer ::
                                        Result :: Err(error) ;
                                    }
                                }
                            }, Err(parser_error) =>
                            {
                                let error = flexpiler :: error :: Error ::
                                gen(parser_error) .
                                propagate(< TestEnumflexpilerDeserializer as
                                          flexpiler :: deserializer :: context
                                          :: VariantTrait < TestEnum,
                                          flexpiler :: common :: rustc ::
                                          Format >> ::
                                          context_variant("TestEnum::DataSimple"))
                                ; return flexpiler :: deserializer :: Result
                                :: Err(error) ;
                            },
                        }
                    }, _ =>
                    {
                        let missing_argument_closure = flexpiler :: common ::
                        rustc :: error :: MissingEnumArgumentSeparator
                        {
                            enum_declaration_found : declaration_result .
                            identifier_string,
                        } ; let error = flexpiler :: Error ::
                        gen(missing_argument_closure) .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> ::
                                  context_variant("TestEnum::DataSimple")) ;
                        return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    },
                } let data = TestEnum :: DataSimple(argument_0_data) ;
                flexpiler :: deserializer :: Result :: DataFound
                {
                    context : flexpiler :: common :: rustc :: deserializer ::
                    Context :: Freestanding, data,
                }
            } "TestEnum::DataComplex" =>
            {
                let context : flexpiler :: common :: rustc :: deserializer ::
                Context = declaration_result . finish . into() ; match context
                {
                    flexpiler :: common :: rustc :: deserializer :: Context ::
                    Freestanding =>
                    {
                        match flexpiler :: parser :: parse :: < flexpiler ::
                        common :: rustc :: block :: DataStart, ReaderType >
                        (reader_mut_ref)
                        {
                            Ok(result) => { }, Err(parser_error) =>
                            {
                                let error = flexpiler :: error :: Error ::
                                gen(parser_error) .
                                propagate(< TestEnumflexpilerDeserializer as
                                          flexpiler :: deserializer :: context
                                          :: VariantTrait < TestEnum,
                                          flexpiler :: common :: rustc ::
                                          Format >> ::
                                          context_variant("TestEnum::DataComplex"))
                                ; return flexpiler :: deserializer :: Result
                                :: Err(error) ;
                            },
                        }
                    }, flexpiler :: common :: rustc :: deserializer :: Context
                    :: DataStart => { }, _ =>
                    {
                        let incompatible_enum_data_type = flexpiler :: common
                        :: rustc :: error :: IncompatibleEnumDataType
                        {
                            enum_declaration_found : declaration_result .
                            identifier_string, enum_data_type_expected :
                            flexpiler :: common :: rustc :: error ::
                            incompatibleenumdataType :: EnumDataType ::
                            Complex, context_found : context,
                        } ; let error = flexpiler :: Error ::
                        gen(incompatible_enum_data_type) .
                        propagate(< TestEnumflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: VariantTrait <
                                  TestEnum, flexpiler :: common :: rustc ::
                                  Format >> ::
                                  context_variant("TestEnum::DataComplex")) ;
                        return flexpiler :: deserializer :: Result ::
                        Err(error) ;
                    }
                } let mut data_context = TestEnumDataComplexflexpilerContext
                :: default() ; loop
                {
                    let field_declaration_string = match flexpiler :: common
                    :: rustc :: block :: DeclarationOrDataEnd ::
                    parse(reader_mut_ref)
                    {
                        Err(parser_error) =>
                        {
                            let error = flexpiler :: error :: Error ::
                            gen(parser_error) .
                            propagate(< TestEnumflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      VariantTrait < TestEnum, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_variant("TestEnum::DataComplex"))
                            ; return flexpiler :: deserializer :: Result ::
                            Err(error) ;
                        },
                        Ok(flexpiler :: common :: rustc :: block ::
                           declaration_or_data_end :: Result :: DataEnd()) =>
                        { break ; },
                        Ok(flexpiler :: common :: rustc :: block ::
                           declaration_or_data_end :: Result ::
                           Declaration(declaration)) => { declaration },
                    } ; let context = match field_declaration_string .
                    as_str()
                    {
                        "a_usize" =>
                        {
                            let result = < usize as flexpiler ::
                            Deserialization < flexpiler :: common :: rustc ::
                            Format >> :: Deserializer ::
                            deserialize(reader_mut_ref) ; match result
                            {
                                flexpiler :: deserializer :: Result ::
                                DataFound { data, context } =>
                                {
                                    data_context . a_usize_option = Some(data)
                                    ; context
                                }, flexpiler :: deserializer :: Result ::
                                NoDataFound { .. } =>
                                {
                                    let unexpected_no_content = flexpiler ::
                                    error :: source :: common ::
                                    UnexpectedNoContent
                                    {
                                        definition_expected : < usize as
                                        flexpiler :: identity :: Trait > ::
                                        definition(),
                                    } ; let error_source_common : flexpiler ::
                                    error :: source :: Common =
                                    unexpected_no_content . into() ; let error
                                    = flexpiler :: Error ::
                                    gen(error_source_common) .
                                    propagate(< TestEnumflexpilerDeserializer
                                              as flexpiler :: deserializer ::
                                              context :: VariantFieldTrait <
                                              TestEnum, flexpiler :: common ::
                                              rustc :: Format >> ::
                                              context_variant_field("TestEnum::DataComplex",
                                                                    "a_usize"))
                                    ; return flexpiler :: deserializer ::
                                    Result :: Err(error) ;
                                }, flexpiler :: deserializer :: Result ::
                                Err(deserializer_error) =>
                                {
                                    let error = deserializer_error .
                                    propagate(< TestEnumflexpilerDeserializer
                                              as flexpiler :: deserializer ::
                                              context :: VariantFieldTrait <
                                              TestEnum, flexpiler :: common ::
                                              rustc :: Format >> ::
                                              context_variant_field("TestEnum::DataComplex",
                                                                    "a_usize"))
                                    ; return flexpiler :: deserializer ::
                                    Result :: Err(error) ;
                                },
                            }
                        } "a_string" =>
                        {
                            let result = < std :: string :: String as
                            flexpiler :: Deserialization < flexpiler :: common
                            :: rustc :: Format >> :: Deserializer ::
                            deserialize(reader_mut_ref) ; match result
                            {
                                flexpiler :: deserializer :: Result ::
                                DataFound { data, context } =>
                                {
                                    data_context . a_string_option =
                                    Some(data) ; context
                                }, flexpiler :: deserializer :: Result ::
                                NoDataFound { .. } =>
                                {
                                    let unexpected_no_content = flexpiler ::
                                    error :: source :: common ::
                                    UnexpectedNoContent
                                    {
                                        definition_expected : < std :: string
                                        :: String as flexpiler :: identity ::
                                        Trait > :: definition(),
                                    } ; let error_source_common : flexpiler ::
                                    error :: source :: Common =
                                    unexpected_no_content . into() ; let error
                                    = flexpiler :: Error ::
                                    gen(error_source_common) .
                                    propagate(< TestEnumflexpilerDeserializer
                                              as flexpiler :: deserializer ::
                                              context :: VariantFieldTrait <
                                              TestEnum, flexpiler :: common ::
                                              rustc :: Format >> ::
                                              context_variant_field("TestEnum::DataComplex",
                                                                    "a_string"))
                                    ; return flexpiler :: deserializer ::
                                    Result :: Err(error) ;
                                }, flexpiler :: deserializer :: Result ::
                                Err(deserializer_error) =>
                                {
                                    let error = deserializer_error .
                                    propagate(< TestEnumflexpilerDeserializer
                                              as flexpiler :: deserializer ::
                                              context :: VariantFieldTrait <
                                              TestEnum, flexpiler :: common ::
                                              rustc :: Format >> ::
                                              context_variant_field("TestEnum::DataComplex",
                                                                    "a_string"))
                                    ; return flexpiler :: deserializer ::
                                    Result :: Err(error) ;
                                },
                            }
                        } _ =>
                        {
                            let unrecognized_field = flexpiler :: common ::
                            rustc :: error :: UnrecognizedFieldDeclaration
                            {
                                field_declaration_found :
                                field_declaration_string,
                                field_declaration_expected_entries : flexpiler
                                :: error :: ExpectedEntries ::
                                from(vec !
                                     [String :: from("a_usize"), String ::
                                      from("a_string"),]),
                            } ; let error = flexpiler :: Error ::
                            gen(unrecognized_field) .
                            propagate(< TestEnumflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      VariantTrait < TestEnum, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_variant("TestEnum::DataComplex"))
                            ; return flexpiler :: deserializer :: Result ::
                            Err(error) ;
                        }
                    } ; match context
                    {
                        flexpiler :: common :: rustc :: deserializer ::
                        Context :: Freestanding =>
                        {
                            match flexpiler :: common :: rustc :: block ::
                            DataEndOrSeparator :: parse(reader_mut_ref)
                            {
                                Ok(flexpiler :: common :: rustc :: block ::
                                   data_end_or_separator :: Result :: DataEnd)
                                => { break ; },
                                Ok(flexpiler :: common :: rustc :: block ::
                                   data_end_or_separator :: Result ::
                                   Separator) => { }, Err(parser_error) =>
                                {
                                    let error = flexpiler :: error :: Error ::
                                    gen(parser_error) .
                                    propagate(< TestEnumflexpilerDeserializer
                                              as flexpiler :: deserializer ::
                                              context :: VariantTrait <
                                              TestEnum, flexpiler :: common ::
                                              rustc :: Format >> ::
                                              context_variant("TestEnum::DataComplex"))
                                    ; return flexpiler :: deserializer ::
                                    Result :: Err(error) ;
                                },
                            }
                        }, flexpiler :: common :: rustc :: deserializer ::
                        Context :: DataEnd => { break ; }, flexpiler :: common
                        :: rustc :: deserializer :: Context :: Separator =>
                        { }, _ =>
                        {
                            let unexpected_entry_finish_context = flexpiler ::
                            common :: rustc :: error ::
                            UnexpectedEntryFinishContext
                            {
                                entry_declaration : field_declaration_string,
                                context_expected : flexpiler :: error ::
                                ExpectedEntries ::
                                from(vec !
                                     [flexpiler :: common :: rustc ::
                                      deserializer :: Context :: DataEnd,
                                      flexpiler :: common :: rustc ::
                                      deserializer :: Context :: Separator,]),
                                context_found : context,
                            } ; let error = flexpiler :: Error ::
                            gen(unexpected_entry_finish_context) .
                            propagate(< TestEnumflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      VariantTrait < TestEnum, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_variant("TestEnum::DataComplex"))
                            ; return flexpiler :: deserializer :: Result ::
                            Err(error) ;
                        }
                    }
                } match data_context . try_into()
                {
                    Ok(data) =>
                    {
                        flexpiler :: deserializer :: Result :: DataFound
                        {
                            context : flexpiler :: common :: rustc ::
                            deserializer :: Context :: Freestanding, data,
                        }
                    }, Err(error) => {
                        flexpiler::deserializer::Result::Err(error)
                    },
                }
            } _ =>
            {
                let incompatible_enum_declaration = flexpiler :: common ::
                rustc :: error :: IncompatibleEnumDeclaration
                {
                    enum_declaration_found : declaration_result .
                    identifier_string, enum_declaration_expected_entries :
                    flexpiler :: error :: ExpectedEntries ::
                    from(vec !
                         [std :: string :: String :: from("TestEnum::NoData"),
                          std :: string :: String ::
                          from("TestEnum::DataEmpty"), std :: string :: String
                          :: from("TestEnum::DataSimple"), std :: string ::
                          String :: from("TestEnum::NoData"),]),
                } ; let error = flexpiler :: Error ::
                gen(incompatible_enum_declaration) .
                propagate(< TestEnumflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: Trait < TestEnum,
                          flexpiler :: common :: rustc :: Format >> ::
                          context()) ; return flexpiler :: deserializer ::
                Result :: Err(error) ;
            }
        } ; return result ;
    }
} impl flexpiler :: Deserialization < flexpiler :: common :: rustc :: Format >
for TestEnum { type Deserializer = TestEnumflexpilerDeserializer ; }













*/


#[test]
fn no_data_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::NoData"
    );

    let test_enum = match TestEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::NoData => {
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::NoData.");
        },
    }
}


#[test]
fn no_data_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::NoData \t\n"
    );

    let test_enum = match TestEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::NoData => {
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::NoData.");
        },
    }
}


#[test]
fn data_empty_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::DataEmpty()"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test_enum_basic_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataEmpty() => {
        },
        _ => {
            assert!(false, "result was not a TestEnum::DataEmpty.");
        },
    }
}


#[test]
fn data_empty_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::DataEmpty( \t\n) \t\n"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "test_enum_basic_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataEmpty() => {
        },
        _ => {
            assert!(false, "result was not a TestEnum::DataEmpty.");
        },
    }
}


#[test]
fn data_simple_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::DataSimple(\"baka\")"
    );

    let test_enum = match TestEnum::deserialize(&mut reader) {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "no_data_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataSimple(argument_0) => {
            assert_eq!(argument_0.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataSimple.");
        },
    }
}


#[test]
fn data_simple_max_formatting_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::DataSimple( \t\n\"baka\" \t\n) \t\n"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "data_simple_max_formatting_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataSimple(argument_0) => {
            assert_eq!(argument_0.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataSimple.");
        },
    }
}


#[test]
fn data_complex_min_formatting_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "TestEnum::DataComplex{a_usize:5,a_string:\"baka\",}"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "data_simple_max_formatting_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataComplex{a_usize, a_string} => {
            assert_eq!(a_usize, 5);
            assert_eq!(a_string.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataComplex.");
        },
    }
}





#[test]
fn data_complex_max_formatting_deserialization_successful() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nTestEnum::DataComplex{ \t\na_usize: \t\n5 \t\n, \t\na_string: \t\n\"baka\" \t\n, \t\n} \t\n"
    );

    let parse_result = TestEnum::deserialize(&mut reader);

    let test_enum = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "data_simple_max_formatting_deserialization_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    match test_enum {
        TestEnum::DataComplex{a_usize, a_string} => {
            assert_eq!(a_usize, 5);
            assert_eq!(a_string.as_str(), "baka");
        },
        _ => {
            assert!(false, "TestEnum result was not a TestEnum::DataComplex.");
        },
    }
}

