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

Last updated: version ```0.1.1```

For above example, the crate generates the following code:

```
pub struct MyPointflexpilerContext
{ 
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
    type Error = flexpiler :: Error ; fn try_into(self) -> std :: result ::
    Result < MyPoint, flexpiler :: Error >
    {
        use flexpiler :: error :: Trait as ErrorTrait ; let x = match self .
        x_option
        {
            Some(value) => value, None =>
            {
                let missing_struct_field = flexpiler :: error ::
                MissingStructField
                {
                    struct_declaration_found : std :: string :: String ::
                    from("MyPoint"), field_declaration_expected : std ::
                    string :: String :: from("x"),
                } ; let error = flexpiler :: Error ::
                gen(missing_struct_field) .
                propagate(MyPointflexpilerDeserializer :: context_general()) ;
                return Err(error) ;
            }
        } ; let y = match self . y_option
        {
            Some(value) => value, None =>
            {
                let missing_struct_field = flexpiler :: error ::
                MissingStructField
                {
                    struct_declaration_found : std :: string :: String ::
                    from("MyPoint"), field_declaration_expected : std ::
                    string :: String :: from("y"),
                } ; let error = flexpiler :: Error ::
                gen(missing_struct_field) .
                propagate(MyPointflexpilerDeserializer :: context_general()) ;
                return Err(error) ;
            }
        } ; Ok(MyPoint { x, y, })
    }
} 


impl MyPointflexpilerDeserializer {
    fn context_field(field_name : & str) -> flexpiler :: error :: Context
    {
        let mut trace = std :: string :: String :: from("MyPoint") ; trace .
        push_str("[") ; trace . push_str(field_name) ; trace . push_str("]") ;
        flexpiler :: error :: Context { trace, }
    } fn context_general() -> flexpiler :: error :: Context
    {
        flexpiler :: error :: Context
        { trace : std :: string :: String :: from("MyPoint"), }
    }
} 


impl flexpiler :: deserializer :: Trait < MyPoint, flexpiler :: common ::
rustc :: deserializer :: Context > for MyPointflexpilerDeserializer
{
    fn deserialize < ReaderType > (reader_mut_ref : & mut ReaderType) -> std
    :: result :: Result < flexpiler :: deserializer :: Result < MyPoint,
    flexpiler :: common :: rustc :: deserializer :: Context >, flexpiler ::
    Error > where ReaderType : flexpiler :: reader :: Trait
    {
        use std :: convert :: TryInto ; use flexpiler :: Deserialization ; use
        flexpiler :: deserializer :: Trait as DeserializerTrait ; use
        flexpiler :: error :: Trait as ErrorTrait ; use flexpiler :: parser ::
        Parse ; let struct_declaration_string = match flexpiler :: common ::
        rustc :: block :: IdentifierWithDataStartFinish ::
        parse(reader_mut_ref)
        {
            Err(parser_error) =>
            {
                let error = flexpiler :: Error :: gen(parser_error) .
                propagate(MyPointflexpilerDeserializer :: context_general()) ;
                return Err(error) ;
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
                        propagate(MyPointflexpilerDeserializer ::
                                  context_general()) ; return Err(error) ;
                    }, Ok(_) => { },
                } declaration
            },
        } ; if struct_declaration_string . as_str() != "MyPoint"
        {
            let incompatible_struct_declaration = flexpiler :: error ::
            IncompatibleStructDeclaration
            {
                struct_declaration_expected : String :: from("MyPoint"),
                struct_declaration_found : struct_declaration_string,
            } ; let error = flexpiler :: Error ::
            gen(incompatible_struct_declaration) .
            propagate(MyPointflexpilerDeserializer :: context_general()) ;
            return Err(error) ;
        } let mut struct_context = MyPointflexpilerContext :: default() ; loop
        {
            let field_declaration_string = match flexpiler :: common :: rustc
            :: block :: DeclarationOrDataEnd :: parse(reader_mut_ref)
            {
                Err(parser_error) =>
                {
                    let error = flexpiler :: error :: Error ::
                    gen(parser_error) .
                    propagate(MyPointflexpilerDeserializer ::
                              context_general()) ; return Err(error) ;
                },
                Ok(flexpiler :: common :: rustc :: block ::
                   declaration_or_data_end :: Result :: DataEnd()) =>
                { break ; },
                Ok(flexpiler :: common :: rustc :: block ::
                   declaration_or_data_end :: Result ::
                   Declaration(declaration)) => { declaration },
            } ; let context = match field_declaration_string . as_str()
            {
                "x" =>
                {
                    let result = < i32 as flexpiler :: Deserialization <
                    flexpiler :: common :: rustc :: Format >> :: Deserializer
                    :: deserialize(reader_mut_ref) ; match result
                    {
                        Ok(value) =>
                        {
                            struct_context . x_option = Some(value . data) ;
                            value . context
                        } Err(error) =>
                        {
                            let error = error .
                            propagate(MyPointflexpilerDeserializer ::
                                      context_field("x")) ; return Err(error)
                            ;
                        }
                    }
                } "y" =>
                {
                    let result = < i32 as flexpiler :: Deserialization <
                    flexpiler :: common :: rustc :: Format >> :: Deserializer
                    :: deserialize(reader_mut_ref) ; match result
                    {
                        Ok(value) =>
                        {
                            struct_context . y_option = Some(value . data) ;
                            value . context
                        } Err(error) =>
                        {
                            let error = error .
                            propagate(MyPointflexpilerDeserializer ::
                                      context_field("y")) ; return Err(error)
                            ;
                        }
                    }
                } _ =>
                {
                    let unrecognized_field = flexpiler :: error ::
                    UnrecognizedFieldDeclaration
                    {
                        field_declaration_found : field_declaration_string,
                        field_declaration_expected_entries : flexpiler ::
                        error :: ExpectedEntries ::
                        from(vec !
                             [String :: from("x"), String :: from("y"),]),
                    } ; let error = flexpiler :: Error ::
                    gen(unrecognized_field) .
                    propagate(MyPointflexpilerDeserializer ::
                              context_general()) ; return Err(error) ;
                }
            } ; match context
            {
                flexpiler :: common :: rustc :: deserializer :: Context ::
                Freestanding =>
                {
                    match flexpiler :: common :: rustc :: block ::
                    DataEndOrSeparator :: parse(reader_mut_ref)
                    {
                        Ok(flexpiler :: common :: rustc :: block ::
                           data_end_or_separator :: Result :: DataEnd) =>
                        { break ; },
                        Ok(flexpiler :: common :: rustc :: block ::
                           data_end_or_separator :: Result :: Separator) =>
                        { }, Err(parser_error) =>
                        {
                            let error = flexpiler :: error :: Error ::
                            gen(parser_error) .
                            propagate(MyPointflexpilerDeserializer ::
                                      context_field(field_declaration_string .
                                                    as_str())) ; return
                            Err(error) ;
                        },
                    }
                }, flexpiler :: common :: rustc :: deserializer :: Context ::
                DataEnd => { break ; }, flexpiler :: common :: rustc ::
                deserializer :: Context :: Separator => { }, _ =>
                {
                    let unexpected_entry_finish_context = flexpiler :: error
                    :: UnexpectedEntryFinishContext
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
                    propagate(MyPointflexpilerDeserializer ::
                              context_general()) ; return Err(error) ;
                }
            }
        } return match struct_context . try_into()
        {
            Ok(data) =>
            {
                Ok(flexpiler :: deserializer :: Result
                   {
                       context : flexpiler :: common :: rustc :: deserializer
                       :: Context :: Freestanding, data,
                   })
            }, Err(error) => { Err(error) },
        }
    }
} 


impl flexpiler :: Deserialization < flexpiler :: common :: rustc :: Format >
for MyPoint {
    type Deserializer = MyPointflexpilerDeserializer; 
}

```


