


#[derive(flexpiler::Deserialize)]
pub struct AutomaticSubTestStruct {
    pub a_usize: usize,
}


/*

impl flexpiler :: identity :: Trait for AutomaticSubTestStruct
{
    fn definition() -> String
    { return std :: string :: String :: from("AutomaticSubTestStruct") ; }
} pub struct AutomaticSubTestStructflexpilerContext
{ a_usize_option : Option < usize >, } pub struct
AutomaticSubTestStructflexpilerDeserializer ; impl Default for
AutomaticSubTestStructflexpilerContext
{
    fn default() -> Self
    { AutomaticSubTestStructflexpilerContext { a_usize_option : None, } }
} impl std :: convert :: TryInto < AutomaticSubTestStruct > for
AutomaticSubTestStructflexpilerContext
{
    type Error = flexpiler :: Error < flexpiler :: common :: rustc :: error ::
    Source > ; fn try_into(self) -> std :: result :: Result <
    AutomaticSubTestStruct, Self :: Error >
    {
        use flexpiler :: deserializer :: Trait as DeserializerTrait ; use
        flexpiler :: deserializer :: context :: Trait as
        DeserializerContextTrait ; use flexpiler :: error :: Trait as
        ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
        ErrorPropagationTrait ; let a_usize = match self . a_usize_option
        {
            Some(value) => value, None =>
            {
                let missing_struct_field = flexpiler :: common :: rustc ::
                error :: MissingStructField
                {
                    struct_declaration_found : std :: string :: String ::
                    from("AutomaticSubTestStruct"), field_declaration_expected
                    : std :: string :: String :: from("a_usize"),
                } ; let error = flexpiler :: Error ::
                gen(missing_struct_field) .
                propagate(< AutomaticSubTestStructflexpilerDeserializer as
                          flexpiler :: deserializer :: context :: Trait <
                          AutomaticSubTestStruct, flexpiler :: common :: rustc
                          :: Format >> :: context()) ; return Err(error) ;
            }
        } ; Ok(AutomaticSubTestStruct { a_usize, })
    }
} impl flexpiler :: deserializer :: Trait < AutomaticSubTestStruct, flexpiler
:: common :: rustc :: Format, > for
AutomaticSubTestStructflexpilerDeserializer where usize : flexpiler ::
Deserialization < flexpiler :: common :: rustc :: Format >
{
    fn deserialize < ReaderType > (reader_mut_ref : & mut ReaderType) ->
    flexpiler :: deserializer :: Result < AutomaticSubTestStruct, flexpiler :: common ::
    rustc :: deserializer :: Context, flexpiler :: Error < flexpiler :: common
    :: rustc :: error :: Source > > where ReaderType : flexpiler :: reader ::
    Trait
    {
        use flexpiler :: deserializer :: Trait as DeserializerTrait ; use
        flexpiler :: deserializer :: context :: Trait as
        DeserializerContextTrait ; use flexpiler :: error :: Trait as
        ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
        ErrorPropagationTrait ; use flexpiler :: identity :: Trait ; use
        flexpiler :: parser :: Parse ; let struct_declaration_string = match
        flexpiler :: common :: rustc :: block :: IdentifierWithDataStartFinish
        :: parse(reader_mut_ref)
        {
            Err(parser_error) =>
            {
                let error = flexpiler :: Error :: gen(parser_error) .
                propagate(< AutomaticSubTestStructflexpilerDeserializer as
                          flexpiler :: deserializer :: context :: Trait <
                          TestStruct, flexpiler :: common :: rustc :: Format
                          >> :: context()) ; return flexpiler :: deserializer
                :: Result :: Err(error) ;
            },
            Ok(flexpiler :: common :: rustc :: block ::
               identifier_with_data_start_finish :: Result ::
               DataStartFinish(declaration)) => { declaration },
            Ok(flexpiler :: common :: rustc :: block ::
               identifier_with_data_start_finish :: Result ::
               Freestanding(declaration)) =>
            {
                match flexpiler :: common :: rustc :: block :: DataStart ::
                parse(reader_mut_ref)
                {
                    Err(parser_error) =>
                    {
                        let error = flexpiler :: Error :: gen(parser_error) .
                        propagate(<
                                  AutomaticSubTestStructflexpilerDeserializer
                                  as flexpiler :: deserializer :: context ::
                                  Trait < TestStruct, flexpiler :: common ::
                                  rustc :: Format >> :: context()) ; return
                        flexpiler :: deserializer :: Result :: Err(error) ;
                    }, Ok(_) => { },
                } declaration
            },
        } ; if struct_declaration_string . as_str() !=
        "AutomaticSubTestStruct"
        {
            let incompatible_struct_declaration = flexpiler :: common :: rustc
            :: error :: IncompatibleStructDeclaration
            {
                struct_declaration_expected : String ::
                from("AutomaticSubTestStruct"), struct_declaration_found :
                struct_declaration_string,
            } ; let error = flexpiler :: Error ::
            gen(incompatible_struct_declaration) .
            propagate(< AutomaticSubTestStructflexpilerDeserializer as
                      flexpiler :: deserializer :: context :: Trait <
                      TestStruct, flexpiler :: common :: rustc :: Format >> ::
                      context()) ; return flexpiler :: deserializer :: Result
            :: Err(error) ;
        } let mut struct_context = AutomaticSubTestStructflexpilerContext ::
        default() ; loop
        {
            let field_declaration_string = match flexpiler :: common :: rustc
            :: block :: DeclarationOrDataEnd :: parse(reader_mut_ref)
            {
                Err(parser_error) =>
                {
                    let error = flexpiler :: error :: Error ::
                    gen(parser_error) .
                    propagate(< AutomaticSubTestStructflexpilerDeserializer as
                              flexpiler :: deserializer :: context :: Trait <
                              TestStruct, flexpiler :: common :: rustc ::
                              Format >> :: context()) ; return flexpiler ::
                    deserializer :: Result :: Err(error) ;
                },
                Ok(flexpiler :: common :: rustc :: block ::
                   declaration_or_data_end :: Result :: DataEnd()) =>
                { break ; },
                Ok(flexpiler :: common :: rustc :: block ::
                   declaration_or_data_end :: Result ::
                   Declaration(declaration)) => { declaration },
            } ; let context = match field_declaration_string . as_str()
            {
                "a_usize" =>
                {
                    let result = < usize as flexpiler :: Deserialization <
                    flexpiler :: common :: rustc :: Format >> :: Deserializer
                    :: deserialize(reader_mut_ref) ; match result
                    {
                        flexpiler :: deserializer :: Result :: DataFound
                        { data, context } =>
                        {
                            struct_context . a_usize_option = Some(data) ;
                            context
                        }, flexpiler :: deserializer :: Result :: NoDataFound
                        { context } =>
                        {
                            let unexpected_no_content = flexpiler :: error ::
                            source :: common :: UnexpectedNoContent
                            {
                                definition_expected : < usize as flexpiler ::
                                identity :: Trait > :: definition(),
                            } ; let error_source_common : flexpiler :: error
                            :: source :: Common = unexpected_no_content .
                            into() ; let error = flexpiler :: Error ::
                            gen(error_source_common) .
                            propagate(<
                                      AutomaticSubTestStructflexpilerDeserializer
                                      as flexpiler :: deserializer :: context
                                      :: FieldTrait < AutomaticSubTestStruct,
                                      flexpiler :: common :: rustc :: Format
                                      >> :: context("a_usize")) ; return
                            flexpiler :: deserializer :: Result :: Err(error)
                            ;
                        }, flexpiler :: deserializer :: Result :: Err(error)
                        =>
                        {
                            let error = error .
                            propagate(<
                                      AutomaticSubTestStructflexpilerDeserializer
                                      as flexpiler :: deserializer :: context
                                      :: FieldTrait < AutomaticSubTestStruct,
                                      flexpiler :: common :: rustc :: Format
                                      >> :: context("a_usize")) ; return
                            flexpiler :: deserializer :: Result :: Err(error)
                            ;
                        }
                    }
                } _ =>
                {
                    let unrecognized_field = flexpiler :: common :: rustc ::
                    error :: UnrecognizedFieldDeclaration
                    {
                        field_declaration_found : field_declaration_string,
                        field_declaration_expected_entries : flexpiler ::
                        error :: ExpectedEntries ::
                        from(vec ! [String :: from("a_usize"),]),
                    } ; let error = flexpiler :: Error ::
                    gen(unrecognized_field) .
                    propagate(< AutomaticSubTestStructflexpilerDeserializer as
                              flexpiler :: deserializer :: context :: Trait <
                              TestStruct, flexpiler :: common :: rustc ::
                              Format >> :: context()) ; return flexpiler ::
                    deserializer :: Result :: Err(error) ;
                }
            } ; if context == flexpiler :: common :: rustc :: deserializer ::
            Context :: Freestanding
            {
                match flexpiler :: common :: rustc :: block ::
                ContextDenominator :: parse(reader_mut_ref)
                {
                    Ok(result) => { context = result . finish . into() ; },
                    Err(parser_error) =>
                    {
                        let error = flexpiler :: Error :: gen(parser_error) .
                        propagate(<
                                  AutomaticSubTestStructflexpilerDeserializer
                                  as flexpiler :: deserializer :: context ::
                                  Trait < TestStruct, flexpiler :: common ::
                                  rustc :: Format >> :: context()) ; return
                        flexpiler :: deserializer :: Result :: Err(error) ;
                    }
                }
            } match context
            {
                flexpiler :: common :: rustc :: deserializer :: Context ::
                DataEnd => { break ; }, flexpiler :: common :: rustc ::
                deserializer :: Context :: Separator => { }, _ =>
                {
                    let unexpected_entry_finish_context = flexpiler :: common
                    :: rustc :: error :: UnexpectedEntryFinishContext
                    {
                        entry_declaration : field_declaration_string,
                        context_expected : flexpiler :: error ::
                        ExpectedEntries ::
                        from(vec !
                             [flexpiler :: common :: rustc :: deserializer ::
                              Context :: DataEnd, flexpiler :: common :: rustc
                              :: deserializer :: Context :: Separator,]),
                        context_found : context,
                    } ; let error = flexpiler :: Error ::
                    gen(unexpected_entry_finish_context) .
                    propagate(< AutomaticSubTestStructflexpilerDeserializer as
                              flexpiler :: deserializer :: context :: Trait <
                              TestStruct, flexpiler :: common :: rustc ::
                              Format >> :: context()) ; return flexpiler ::
                    deserializer :: Result :: Err(error) ;
                }
            }
        } return match < AutomaticSubTestStructflexpilerContext as std ::
        convert :: TryInto < AutomaticSubTestStruct >> ::
        try_into(struct_context)
        {
            Ok(data) =>
            {
                flexpiler :: deserializer :: Result :: DataFound
                {
                    context : flexpiler :: common :: rustc :: deserializer ::
                    Context :: Freestanding, data,
                }
            }, Err(error) =>
            { flexpiler :: deserializer :: Result :: Err(error) },
        }
    }
} impl flexpiler :: Deserialization < flexpiler :: common :: rustc :: Format >
for AutomaticSubTestStruct
{ type Deserializer = AutomaticSubTestStructflexpilerDeserializer ; }




*/



#[derive(flexpiler::Deserialize)]
pub struct AutomaticTestStruct {
    pub a_i32: i32,
    pub a_usize: usize,
    pub a_string: std::string::String,
    pub a_sub: AutomaticSubTestStruct,
}



/*




impl flexpiler :: identity :: Trait for AutomaticTestStruct
{
    fn definition() -> String
    { return std :: string :: String :: from("AutomaticTestStruct") ; }
} pub struct AutomaticTestStructflexpilerContext
{
    a_i32_option : Option < i32 >, a_usize_option : Option < usize >,
    a_string_option : Option < std :: string :: String >, a_sub_option :
Option < AutomaticSubTestStruct >,
} pub struct AutomaticTestStructflexpilerDeserializer ; impl Default for
AutomaticTestStructflexpilerContext
{
    fn default() -> Self
    {
        AutomaticTestStructflexpilerContext
        {
            a_i32_option : None, a_usize_option : None, a_string_option :
        None, a_sub_option : None,
        }
    }
} impl std :: convert :: TryInto < AutomaticTestStruct > for
AutomaticTestStructflexpilerContext
{
    type Error = flexpiler :: Error < flexpiler :: common :: rustc :: error ::
    Source > ; fn try_into(self) -> std :: result :: Result <
    AutomaticTestStruct, Self :: Error >
{
    use flexpiler :: deserializer :: Trait as DeserializerTrait ; use
flexpiler :: deserializer :: context :: Trait as
DeserializerContextTrait ; use flexpiler :: error :: Trait as
ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
ErrorPropagationTrait ; let a_i32 = match self . a_i32_option
{
    Some(value) => value, None =>
    {
        let missing_struct_field = flexpiler :: common :: rustc ::
        error :: MissingStructField
        {
            struct_declaration_found : std :: string :: String ::
            from("AutomaticTestStruct"), field_declaration_expected :
        std :: string :: String :: from("a_i32"),
        } ; let error = flexpiler :: Error ::
    gen(missing_struct_field) .
        propagate(< AutomaticTestStructflexpilerDeserializer as
        flexpiler :: deserializer :: context :: Trait <
            AutomaticTestStruct, flexpiler :: common :: rustc ::
            Format >> :: context()) ; return Err(error) ;
    }
} ; let a_usize = match self . a_usize_option
{
    Some(value) => value, None =>
    {
        let missing_struct_field = flexpiler :: common :: rustc ::
        error :: MissingStructField
        {
            struct_declaration_found : std :: string :: String ::
            from("AutomaticTestStruct"), field_declaration_expected :
        std :: string :: String :: from("a_usize"),
        } ; let error = flexpiler :: Error ::
    gen(missing_struct_field) .
        propagate(< AutomaticTestStructflexpilerDeserializer as
        flexpiler :: deserializer :: context :: Trait <
            AutomaticTestStruct, flexpiler :: common :: rustc ::
            Format >> :: context()) ; return Err(error) ;
    }
} ; let a_string = match self . a_string_option
{
    Some(value) => value, None =>
    {
        let missing_struct_field = flexpiler :: common :: rustc ::
        error :: MissingStructField
        {
            struct_declaration_found : std :: string :: String ::
            from("AutomaticTestStruct"), field_declaration_expected :
        std :: string :: String :: from("a_string"),
        } ; let error = flexpiler :: Error ::
    gen(missing_struct_field) .
        propagate(< AutomaticTestStructflexpilerDeserializer as
        flexpiler :: deserializer :: context :: Trait <
            AutomaticTestStruct, flexpiler :: common :: rustc ::
            Format >> :: context()) ; return Err(error) ;
    }
} ; let a_sub = match self . a_sub_option
{
    Some(value) => value, None =>
    {
        let missing_struct_field = flexpiler :: common :: rustc ::
        error :: MissingStructField
        {
            struct_declaration_found : std :: string :: String ::
            from("AutomaticTestStruct"), field_declaration_expected :
        std :: string :: String :: from("a_sub"),
        } ; let error = flexpiler :: Error ::
    gen(missing_struct_field) .
        propagate(< AutomaticTestStructflexpilerDeserializer as
        flexpiler :: deserializer :: context :: Trait <
            AutomaticTestStruct, flexpiler :: common :: rustc ::
            Format >> :: context()) ; return Err(error) ;
    }
} ; Ok(AutomaticTestStruct { a_i32, a_usize, a_string, a_sub, })
}
} impl flexpiler :: deserializer :: Trait < AutomaticTestStruct, flexpiler ::
common :: rustc :: Format, > for AutomaticTestStructflexpilerDeserializer
    where i32 : flexpiler :: Deserialization < flexpiler :: common :: rustc ::
    Format >, usize : flexpiler :: Deserialization < flexpiler :: common :: rustc
:: Format >, std :: string :: String : flexpiler :: Deserialization < flexpiler
:: common :: rustc :: Format >, AutomaticSubTestStruct : flexpiler ::
Deserialization < flexpiler :: common :: rustc :: Format >,
{
fn deserialize < ReaderType > (reader_mut_ref : & mut ReaderType) ->
flexpiler :: deserializer :: Result < AutomaticTestStruct, flexpiler ::
common :: rustc :: deserializer :: Context, flexpiler :: Error < flexpiler
:: common :: rustc :: error :: Source > > where ReaderType : flexpiler ::
reader :: Trait
{
use flexpiler :: deserializer :: Trait as DeserializerTrait ; use
flexpiler :: deserializer :: context :: Trait as
DeserializerContextTrait ; use flexpiler :: error :: Trait as
ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
ErrorPropagationTrait ; use flexpiler :: identity :: Trait ; use
flexpiler :: parser :: Parse ; let struct_declaration_string = match
flexpiler :: common :: rustc :: block :: IdentifierWithDataStartFinish
:: parse(reader_mut_ref)
{
Err(parser_error) =>
{
let error = flexpiler :: Error :: gen(parser_error) .
propagate(< AutomaticTestStructflexpilerDeserializer as
flexpiler :: deserializer :: context :: Trait <
AutomaticTestStruct, flexpiler :: common :: rustc ::
Format >> :: context()) ; return flexpiler ::
deserializer :: Result :: Err(error) ;
},
Ok(flexpiler :: common :: rustc :: block ::
identifier_with_data_start_finish :: Result ::
DataStartFinish(declaration)) => { declaration },
Ok(flexpiler :: common :: rustc :: block ::
identifier_with_data_start_finish :: Result ::
Freestanding(declaration)) =>
{
match flexpiler :: common :: rustc :: block :: DataStart ::
parse(reader_mut_ref)
{
Err(parser_error) =>
{
let error = flexpiler :: Error :: gen(parser_error) .
propagate(< AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context ::
Trait < AutomaticTestStruct, flexpiler ::
common :: rustc :: Format >> :: context()) ;
return flexpiler :: deserializer :: Result ::
Err(error) ;
}, Ok(_) => { },
} declaration
},
} ; if struct_declaration_string . as_str() != "AutomaticTestStruct"
{
let incompatible_struct_declaration = flexpiler :: common :: rustc
:: error :: IncompatibleStructDeclaration
{
struct_declaration_expected : String ::
from("AutomaticTestStruct"), struct_declaration_found :
struct_declaration_string,
} ; let error = flexpiler :: Error ::
gen(incompatible_struct_declaration) .
propagate(< AutomaticTestStructflexpilerDeserializer as flexpiler
:: deserializer :: context :: Trait <
AutomaticTestStruct, flexpiler :: common :: rustc ::
Format >> :: context()) ; return flexpiler ::
deserializer :: Result :: Err(error) ;
} let mut struct_context = AutomaticTestStructflexpilerContext ::
default() ; loop
{
let field_declaration_string = match flexpiler :: common :: rustc
:: block :: DeclarationOrDataEnd :: parse(reader_mut_ref)
{
Err(parser_error) =>
{
let error = flexpiler :: error :: Error ::
gen(parser_error) .
propagate(< AutomaticTestStructflexpilerDeserializer as
flexpiler :: deserializer :: context :: Trait <
AutomaticTestStruct, flexpiler :: common ::
rustc :: Format >> :: context()) ; return
flexpiler :: deserializer :: Result :: Err(error) ;
},
Ok(flexpiler :: common :: rustc :: block ::
declaration_or_data_end :: Result :: DataEnd()) =>
{ break ; },
Ok(flexpiler :: common :: rustc :: block ::
declaration_or_data_end :: Result ::
Declaration(declaration)) => { declaration },
} ; let mut context = match field_declaration_string . as_str()
{
"a_i32" =>
{
let result = < i32 as flexpiler :: Deserialization <
flexpiler :: common :: rustc :: Format >> :: Deserializer
:: deserialize(reader_mut_ref) ; match result
{
flexpiler :: deserializer :: Result :: DataFound
{ data, context } =>
{
struct_context . a_i32_option = Some(data) ;
context
}, flexpiler :: deserializer :: Result :: NoDataFound
{ context } =>
{
let unexpected_no_content = flexpiler :: error ::
source :: common :: UnexpectedNoContent
{
definition_expected : < i32 as flexpiler ::
identity :: Trait > :: definition(),
} ; let error_source_common : flexpiler :: error
:: source :: Common = unexpected_no_content .
into() ; let error = flexpiler :: Error ::
gen(error_source_common) .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_i32")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}, flexpiler :: deserializer :: Result :: Err(error)
=>
{
let error = error .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_i32")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}
}
} "a_usize" =>
{
let result = < usize as flexpiler :: Deserialization <
flexpiler :: common :: rustc :: Format >> :: Deserializer
:: deserialize(reader_mut_ref) ; match result
{
flexpiler :: deserializer :: Result :: DataFound
{ data, context } =>
{
struct_context . a_usize_option = Some(data) ;
context
}, flexpiler :: deserializer :: Result :: NoDataFound
{ context } =>
{
let unexpected_no_content = flexpiler :: error ::
source :: common :: UnexpectedNoContent
{
definition_expected : < usize as flexpiler ::
identity :: Trait > :: definition(),
} ; let error_source_common : flexpiler :: error
:: source :: Common = unexpected_no_content .
into() ; let error = flexpiler :: Error ::
gen(error_source_common) .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_usize")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}, flexpiler :: deserializer :: Result :: Err(error)
=>
{
let error = error .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_usize")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}
}
} "a_string" =>
{
let result = < std :: string :: String as flexpiler ::
Deserialization < flexpiler :: common :: rustc :: Format
>> :: Deserializer :: deserialize(reader_mut_ref) ; match
result
{
flexpiler :: deserializer :: Result :: DataFound
{ data, context } =>
{
struct_context . a_string_option = Some(data) ;
context
}, flexpiler :: deserializer :: Result :: NoDataFound
{ context } =>
{
let unexpected_no_content = flexpiler :: error ::
source :: common :: UnexpectedNoContent
{
definition_expected : < std :: string ::
String as flexpiler :: identity :: Trait > ::
definition(),
} ; let error_source_common : flexpiler :: error
:: source :: Common = unexpected_no_content .
into() ; let error = flexpiler :: Error ::
gen(error_source_common) .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_string")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}, flexpiler :: deserializer :: Result :: Err(error)
=>
{
let error = error .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_string")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}
}
} "a_sub" =>
{
let result = < AutomaticSubTestStruct as flexpiler ::
Deserialization < flexpiler :: common :: rustc :: Format
>> :: Deserializer :: deserialize(reader_mut_ref) ; match
result
{
flexpiler :: deserializer :: Result :: DataFound
{ data, context } =>
{
struct_context . a_sub_option = Some(data) ;
context
}, flexpiler :: deserializer :: Result :: NoDataFound
{ context } =>
{
let unexpected_no_content = flexpiler :: error ::
source :: common :: UnexpectedNoContent
{
definition_expected : < AutomaticSubTestStruct
as flexpiler :: identity :: Trait > ::
definition(),
} ; let error_source_common : flexpiler :: error
:: source :: Common = unexpected_no_content .
into() ; let error = flexpiler :: Error ::
gen(error_source_common) .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_sub")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}, flexpiler :: deserializer :: Result :: Err(error)
=>
{
let error = error .
propagate(<
AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context
:: FieldTrait < AutomaticTestStruct,
flexpiler :: common :: rustc :: Format
>> :: context("a_sub")) ; return
flexpiler :: deserializer :: Result :: Err(error)
;
}
}
} _ =>
{
let unrecognized_field = flexpiler :: common :: rustc ::
error :: UnrecognizedFieldDeclaration
{
field_declaration_found : field_declaration_string,
field_declaration_expected_entries : flexpiler ::
error :: ExpectedEntries ::
from(vec !
[String :: from("a_i32"), String ::
from("a_usize"), String :: from("a_string"),
String :: from("a_sub"),]),
} ; let error = flexpiler :: Error ::
gen(unrecognized_field) .
propagate(< AutomaticTestStructflexpilerDeserializer as
flexpiler :: deserializer :: context :: Trait <
AutomaticTestStruct, flexpiler :: common ::
rustc :: Format >> :: context()) ; return
flexpiler :: deserializer :: Result :: Err(error) ;
}
} ; if context == flexpiler :: common :: rustc :: deserializer ::
Context :: Freestanding
{
match flexpiler :: common :: rustc :: block ::
ContextDenominator :: parse(reader_mut_ref)
{
Ok(result) => { context = result . finish . into() ; },
Err(parser_error) =>
{
let error = flexpiler :: Error :: gen(parser_error) .
propagate(< AutomaticTestStructflexpilerDeserializer
as flexpiler :: deserializer :: context ::
Trait < AutomaticTestStruct, flexpiler ::
common :: rustc :: Format >> :: context()) ;
return flexpiler :: deserializer :: Result ::
Err(error) ;
}
}
} match context
{
flexpiler :: common :: rustc :: deserializer :: Context ::
DataEnd => { break ; }, flexpiler :: common :: rustc ::
deserializer :: Context :: Separator => { }, _ =>
{
let unexpected_entry_finish_context = flexpiler :: common
:: rustc :: error :: UnexpectedEntryFinishContext
{
entry_declaration : field_declaration_string,
context_expected : flexpiler :: error ::
ExpectedEntries ::
from(vec !
[flexpiler :: common :: rustc :: deserializer ::
Context :: DataEnd, flexpiler :: common :: rustc
:: deserializer :: Context :: Separator,]),
context_found : context,
} ; let error = flexpiler :: Error ::
gen(unexpected_entry_finish_context) .
propagate(< AutomaticTestStructflexpilerDeserializer as
flexpiler :: deserializer :: context :: Trait <
AutomaticTestStruct, flexpiler :: common ::
rustc :: Format >> :: context()) ; return
flexpiler :: deserializer :: Result :: Err(error) ;
}
}
} return match < AutomaticTestStructflexpilerContext as std :: convert
:: TryInto < AutomaticTestStruct >> :: try_into(struct_context)
{
Ok(data) =>
{
flexpiler :: deserializer :: Result :: DataFound
{
context : flexpiler :: common :: rustc :: deserializer ::
Context :: Freestanding, data,
}
}, Err(error) =>
{ flexpiler :: deserializer :: Result :: Err(error) },
}
}
} impl flexpiler :: Deserialization < flexpiler :: common :: rustc :: Format >
for AutomaticTestStruct
{ type Deserializer = AutomaticTestStructflexpilerDeserializer ; }





*/


#[test]
fn automaticsubteststruct_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "AutomaticSubTestStruct{a_usize:60}"
    );

    let parse_result = AutomaticSubTestStruct::deserialize(&mut reader);

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


#[test]
fn automaticsubteststruct_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nAutomaticSubTestStruct \t\n{ \t\na_usize: \t\n60 \t\n} \t\n"
    );

    let parse_result = AutomaticSubTestStruct::deserialize(&mut reader);

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


#[test]
fn automatic_test_struct_min_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        "AutomaticTestStruct{a_string:\"Hello\",a_i32:-34,a_usize:50,a_sub:AutomaticSubTestStruct{a_usize:67}}"
    );

    let parse_result = AutomaticTestStruct::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_automatic_test_struct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_string, "Hello",
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_string,
               "Hello");

    assert_eq!(test_struct.a_usize, 50,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_usize,
               50);

    assert_eq!(test_struct.a_i32, -34,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               -34);

    assert_eq!(test_struct.a_sub.a_usize, 67,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               67);
}


#[test]
fn automatic_test_struct_max_formatting() {
    use flexpiler::Deserialize;
    use flexpiler::common::reader;

    let mut reader = reader::String::from(
        " \t\nAutomaticTestStruct \t\n{ \t\na_string: \t\n\"Hello\" \t\n, \t\na_i32: \t\n-34 \t\n, \t\na_usize: \t\n50 \t\n, \t\na_sub: \t\nAutomaticSubTestStruct \t\n{ \t\na_usize: \t\n67 \t\n} \t\n} \t\n"
    );

    let parse_result = AutomaticTestStruct::deserialize(&mut reader);

    let test_struct = match parse_result {
        Ok(value) => value,
        Err(error) => {
            assert!(false, "simple_automatic_test_struct_basic_serialisation_successful() test ended in a failed deserialization:\n{}", error);
            return;
        }
    };

    assert_eq!(test_struct.a_string, "Hello",
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_string,
               "Hello");

    assert_eq!(test_struct.a_usize, 50,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_usize,
               50);

    assert_eq!(test_struct.a_i32, -34,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               -34);

    assert_eq!(test_struct.a_sub.a_usize, 67,
               "simple_automatic_test_struct_basic_serialisation_successful() deserialised value had unexpected string value {}, expected {}",
               test_struct.a_i32,
               67);
}
