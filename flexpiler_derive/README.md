# Flexpiler_derive

This is a sister crate of ```flexpiler```, a crate to provide customized deserialization (and eventually serialization).


## What this crate does

This project provides a ```#[derive]``` macro. This macro generates code such that the struct / enum being derived gains an impl of ```flexpiler::Deserialize```. Intended usage of the crate is by adding a dependency to the ```flexpiler``` crate and using the macro as re-exported from it like so:

```rust
#[derive(flexpiler::Deserialize)]
struct MyPoint {
    x: i32,
    y: i32,
}
```


## Example output of the crate

Last updated: version ```0.2.0```

For above example, the crate generates the following code:

```
impl flexpiler :: identity :: Trait for MyPoint {
    fn definition() -> String
    { return std :: string :: String :: from("MyPoint") ; }
}
 
 
pub struct MyPointflexpilerContext{
    x_option : Option < i32 >,
    y_option : Option < i32 >, 
} 


pub struct MyPointflexpilerDeserializer ;
 
 
impl Default for MyPointflexpilerContext
{
    fn default() -> Self {
        MyPointflexpilerContext { 
            x_option : None, 
            y_option : None, 
        } 
    }
} 


impl std :: convert :: TryInto < MyPoint > for MyPointflexpilerContext
{
    type Error = flexpiler :: Error < flexpiler :: common :: rustc :: error ::
    Source > ; fn try_into(self) -> std :: result :: Result < MyPoint, Self ::
    Error >
    {
        use flexpiler :: deserializer :: Trait as DeserializerTrait ; use
        flexpiler :: deserializer :: context :: Trait as
        DeserializerContextTrait ; use flexpiler :: error :: Trait as
        ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
        ErrorPropagationTrait ; let x = match self . x_option
        {
            Some(value) => value, None =>
            {
                let missing_struct_field = flexpiler :: common :: rustc ::
                error :: MissingStructField
                {
                    struct_declaration_found : std :: string :: String ::
                    from("MyPoint"), field_declaration_expected : std ::
                    string :: String :: from("x"),
                } ; let error = flexpiler :: Error ::
                gen(missing_struct_field) .
                propagate(< MyPointflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: Trait < MyPoint,
                          flexpiler :: common :: rustc :: Format >> ::
                          context()) ; return Err(error) ;
            }
        } ; let y = match self . y_option
        {
            Some(value) => value, None =>
            {
                let missing_struct_field = flexpiler :: common :: rustc ::
                error :: MissingStructField
                {
                    struct_declaration_found : std :: string :: String ::
                    from("MyPoint"), field_declaration_expected : std ::
                    string :: String :: from("y"),
                } ; let error = flexpiler :: Error ::
                gen(missing_struct_field) .
                propagate(< MyPointflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: Trait < MyPoint,
                          flexpiler :: common :: rustc :: Format >> ::
                          context()) ; return Err(error) ;
            }
        } ; Ok(MyPoint { x, y, })
    }
} impl flexpiler :: deserializer :: Trait < MyPoint, flexpiler :: common ::
rustc :: Format, > for MyPointflexpilerDeserializer where i32 : flexpiler ::
Deserialization < flexpiler :: common :: rustc :: Format >, i32 : flexpiler ::
Deserialization < flexpiler :: common :: rustc :: Format >,
{
    fn deserialize < ReaderType > (reader_mut_ref : & mut ReaderType) ->
    flexpiler :: deserializer :: Result < MyPoint, flexpiler :: common ::
    rustc :: deserializer :: Context, flexpiler :: Error < flexpiler :: common
    :: rustc :: error :: Source > > where ReaderType : flexpiler :: reader ::
    Trait
    {
        use flexpiler :: deserializer :: Trait as DeserializerTrait ; use
        flexpiler :: deserializer :: context :: Trait as
        DeserializerContextTrait ; use flexpiler :: error :: Trait as
        ErrorTrait ; use flexpiler :: error :: propagation :: Trait as
        ErrorPropagationTrait ; use flexpiler :: identity :: Trait ; use
        flexpiler :: parser :: Parse ; let(identifier_data, identifier_finish)
        = match flexpiler :: common :: rustc :: block :: Identifier ::
        parse(reader_mut_ref)
        {
            Ok(flexpiler :: common :: rustc :: block :: identifier :: Result
               :: NoDataFound { finish }) =>
            {
                return flexpiler :: deserializer :: Result :: NoDataFound
                { context : finish . into() } ;
            },
            Ok(flexpiler :: common :: rustc :: block :: identifier :: Result
               :: DataFound { data, finish }) => { (data, finish) },
            Err(parser_error) =>
            {
                let error = flexpiler :: Error :: gen(parser_error) .
                propagate(< MyPointflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: Trait < MyPoint,
                          flexpiler :: common :: rustc :: Format >> ::
                          context()) ; return flexpiler :: deserializer ::
                Result :: Err(error) ;
            }
        } ; let mut context : flexpiler :: common :: rustc :: deserializer ::
        Context = identifier_finish . into() ; if context == flexpiler ::
        common :: rustc :: deserializer :: Context :: Freestanding
        {
            context = match flexpiler :: common :: rustc :: block ::
            ContextDenominator :: parse(reader_mut_ref)
            {
                Ok(result) => { result . finish . into() }, Err(parser_error)
                =>
                {
                    let error = flexpiler :: Error :: gen(parser_error) .
                    propagate(< MyPointflexpilerDeserializer as flexpiler ::
                              deserializer :: context :: Trait < MyPoint,
                              flexpiler :: common :: rustc :: Format >> ::
                              context()) ; return flexpiler :: deserializer ::
                    Result :: Err(error) ;
                },
            }
        } match context
        {
            flexpiler :: common :: rustc :: deserializer :: Context ::
            DataStart => { }, _ =>
            {
                let unexpected_context = flexpiler :: common :: rustc :: error
                :: UnexpectedContext
                {
                    context_found : context, context_expected : flexpiler ::
                    error :: ExpectedEntries ::
                    from(vec !
                         [flexpiler :: common :: rustc :: deserializer ::
                          Context :: DataStart,]),
                } ; let error = flexpiler :: Error :: gen(unexpected_context)
                .
                propagate(< MyPointflexpilerDeserializer as flexpiler ::
                          deserializer :: context :: Trait < MyPoint,
                          flexpiler :: common :: rustc :: Format >> ::
                          context()) ; return flexpiler :: deserializer ::
                Result :: Err(error) ;
            },
        } if identifier_data . as_str() != "MyPoint"
        {
            let incompatible_struct_declaration = flexpiler :: common :: rustc
            :: error :: IncompatibleStructDeclaration
            {
                struct_declaration_expected : String :: from("MyPoint"),
                struct_declaration_found : identifier_data,
            } ; let error = flexpiler :: Error ::
            gen(incompatible_struct_declaration) .
            propagate(< MyPointflexpilerDeserializer as flexpiler ::
                      deserializer :: context :: Trait < MyPoint, flexpiler ::
                      common :: rustc :: Format >> :: context()) ; return
            flexpiler :: deserializer :: Result :: Err(error) ;
        } let mut struct_context = MyPointflexpilerContext :: default() ; loop
        {
            let field_declaration_string = match flexpiler :: common :: rustc
            :: block :: DeclarationOrDataEnd :: parse(reader_mut_ref)
            {
                Err(parser_error) =>
                {
                    let error = flexpiler :: error :: Error ::
                    gen(parser_error) .
                    propagate(< MyPointflexpilerDeserializer as flexpiler ::
                              deserializer :: context :: Trait < MyPoint,
                              flexpiler :: common :: rustc :: Format >> ::
                              context()) ; return flexpiler :: deserializer ::
                    Result :: Err(error) ;
                },
                Ok(flexpiler :: common :: rustc :: block ::
                   declaration_or_data_end :: Result :: DataEnd()) =>
                { break ; },
                Ok(flexpiler :: common :: rustc :: block ::
                   declaration_or_data_end :: Result ::
                   Declaration(declaration)) => { declaration },
            } ; let mut context = match field_declaration_string . as_str()
            {
                "x" =>
                {
                    let result = < i32 as flexpiler :: Deserialization <
                    flexpiler :: common :: rustc :: Format >> :: Deserializer
                    :: deserialize(reader_mut_ref) ; match result
                    {
                        flexpiler :: deserializer :: Result :: DataFound
                        { data, context } =>
                        { struct_context . x_option = Some(data) ; context },
                        flexpiler :: deserializer :: Result :: NoDataFound
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
                            propagate(< MyPointflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      FieldTrait < MyPoint, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_field("x")) ; return flexpiler
                            :: deserializer :: Result :: Err(error) ;
                        }, flexpiler :: deserializer :: Result :: Err(error)
                        =>
                        {
                            let error = error .
                            propagate(< MyPointflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      FieldTrait < MyPoint, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_field("x")) ; return flexpiler
                            :: deserializer :: Result :: Err(error) ;
                        }
                    }
                } "y" =>
                {
                    let result = < i32 as flexpiler :: Deserialization <
                    flexpiler :: common :: rustc :: Format >> :: Deserializer
                    :: deserialize(reader_mut_ref) ; match result
                    {
                        flexpiler :: deserializer :: Result :: DataFound
                        { data, context } =>
                        { struct_context . y_option = Some(data) ; context },
                        flexpiler :: deserializer :: Result :: NoDataFound
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
                            propagate(< MyPointflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      FieldTrait < MyPoint, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_field("y")) ; return flexpiler
                            :: deserializer :: Result :: Err(error) ;
                        }, flexpiler :: deserializer :: Result :: Err(error)
                        =>
                        {
                            let error = error .
                            propagate(< MyPointflexpilerDeserializer as
                                      flexpiler :: deserializer :: context ::
                                      FieldTrait < MyPoint, flexpiler ::
                                      common :: rustc :: Format >> ::
                                      context_field("y")) ; return flexpiler
                            :: deserializer :: Result :: Err(error) ;
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
                             [String :: from("x"), String :: from("y"),]),
                    } ; let error = flexpiler :: Error ::
                    gen(unrecognized_field) .
                    propagate(< MyPointflexpilerDeserializer as flexpiler ::
                              deserializer :: context :: Trait < MyPoint,
                              flexpiler :: common :: rustc :: Format >> ::
                              context()) ; return flexpiler :: deserializer ::
                    Result :: Err(error) ;
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
                        propagate(< MyPointflexpilerDeserializer as flexpiler
                                  :: deserializer :: context :: Trait <
                                  MyPoint, flexpiler :: common :: rustc ::
                                  Format >> :: context()) ; return flexpiler
                        :: deserializer :: Result :: Err(error) ;
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
                    propagate(< MyPointflexpilerDeserializer as flexpiler ::
                              deserializer :: context :: Trait < MyPoint,
                              flexpiler :: common :: rustc :: Format >> ::
                              context()) ; return flexpiler :: deserializer ::
                    Result :: Err(error) ;
                }
            }
        } return match < MyPointflexpilerContext as std :: convert :: TryInto
        < MyPoint >> :: try_into(struct_context)
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
} 


impl flexpiler :: Deserialization < flexpiler :: common :: rustc :: Format >
for MyPoint {
    type Deserializer = MyPointflexpilerDeserializer ; 
}

```


