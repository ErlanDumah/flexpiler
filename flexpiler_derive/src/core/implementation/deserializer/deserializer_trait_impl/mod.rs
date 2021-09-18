pub mod data_argumented_deserialization;
pub use data_argumented_deserialization::DataArgumentedDeserialization;
pub mod data_context_deserialization;
pub use data_context_deserialization::DataContextDeserialization;

use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct DeserializerTraitImpl;


pub struct ParameterStruct<'a> {
    pub struct_definition_ref: &'a definition::Struct,
    pub struct_intermediary_ref: &'a intermediary::Struct<'a>,
    pub struct_data_context_intermediary_ref: &'a intermediary::DataContext,
    pub struct_deserializer_intermediary_ref: &'a intermediary::deserializer::Struct,
}


pub struct ParameterEnum<'a> {
    pub enum_definition_ref: &'a definition::Enum,
    pub enum_intermediary_ref: &'a intermediary::Enum<'a>,
    pub enum_variable_complex_data_context_intermediary_vec_ref: &'a Vec<intermediary::DataContext>,
    pub enum_deserializer_intermediary_ref: &'a intermediary::deserializer::Enum,
}


impl<'a> implementation::Trait<ParameterStruct<'a>> for DeserializerTraitImpl {
    fn gen(parameter: ParameterStruct) -> proc_macro2::TokenStream {
        let generics = if !parameter.struct_definition_ref.data_struct.fields.is_empty() {
            let mut generic_vec = Vec::new();
            for type_ref in parameter.struct_intermediary_ref.field_type_vec.iter() {
                generic_vec.push(quote!{ #type_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format>, });
            }
            quote!{
                where #(#generic_vec)*
            }
        } else {
            quote!{}
        };

        let struct_definition_ident_ref = &parameter.struct_definition_ref.ident;
        let struct_intermediary_ident_string_ref = &parameter.struct_intermediary_ref.ident_string;
        let struct_intermediary_field_string_vec_ref = &parameter.struct_intermediary_ref.field_string_vec;
        let struct_intermediary_field_type_vec_ref = &parameter.struct_intermediary_ref.field_type_vec;
        let struct_deserializer_intermediary_ident_ref = &parameter.struct_deserializer_intermediary_ref.ident;
        let struct_data_context_intermediary_ident_ref = &parameter.struct_data_context_intermediary_ref.ident;
        let struct_data_context_intermediary_field_ident_vec_ref = &parameter.struct_data_context_intermediary_ref.field_ident_vec;

        quote!{
            impl flexpiler::deserializer::Trait<
                #struct_definition_ident_ref,
                flexpiler::common::rustc::Format,
            > for #struct_deserializer_intermediary_ident_ref
            #generics {
                fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                    -> flexpiler::deserializer::Result<
                        #struct_definition_ident_ref,
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

                    let struct_declaration_string = match flexpiler::common::rustc::block::IdentifierWithDataStartFinish::parse(reader_mut_ref) {
                        Err(parser_error) => {
                            let error = flexpiler::Error::gen(parser_error)
                                .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                            return flexpiler::deserializer::Result::Err(error);
                        },
                        Ok(flexpiler::common::rustc::block::identifier_with_data_start_finish::Result::DataStartFinish(declaration)) => {
                            declaration
                        },
                        Ok(flexpiler::common::rustc::block::identifier_with_data_start_finish::Result::Freestanding(declaration)) => {
                            match flexpiler::common::rustc::block::DataStart::parse(reader_mut_ref) {
                                Err(parser_error) => {
                                    let error = flexpiler::Error::gen(parser_error)
                                        .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                                    return flexpiler::deserializer::Result::Err(error);
                                },
                                Ok(_) => {
                                    // successfully parsed data start
                                },
                            }

                            declaration
                        },
                    };

                    if struct_declaration_string.as_str() != #struct_intermediary_ident_string_ref {
                        let incompatible_struct_declaration = flexpiler::common::rustc::error::IncompatibleStructDeclaration {
                            struct_declaration_expected: String::from(#struct_intermediary_ident_string_ref),
                            struct_declaration_found: struct_declaration_string,
                        };
                        let error = flexpiler::Error::gen(incompatible_struct_declaration)
                            .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                        return flexpiler::deserializer::Result::Err(error);
                    }

                    let mut struct_context = #struct_data_context_intermediary_ident_ref::default();

                    loop {
                        let field_declaration_string = match flexpiler::common::rustc::block::DeclarationOrDataEnd::parse(reader_mut_ref) {
                            Err(parser_error) => {
                                let error = flexpiler::error::Error::gen(parser_error)
                                    .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                                return flexpiler::deserializer::Result::Err(error);
                            },
                            Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::DataEnd()) => {
                                break;
                            },
                            Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::Declaration(declaration)) => {
                                declaration
                            },
                        };

                        let mut context = match field_declaration_string.as_str() {
                            #(#struct_intermediary_field_string_vec_ref => {
                                let result = <#struct_intermediary_field_type_vec_ref as flexpiler::Deserialization<flexpiler::common::rustc::Format>>
                                            ::Deserializer::deserialize(reader_mut_ref);
                                match result {
                                    flexpiler::deserializer::Result::DataFound{ data, context } => {
                                        struct_context.#struct_data_context_intermediary_field_ident_vec_ref = Some(data);
                                        context
                                    },
                                    flexpiler::deserializer::Result::NoDataFound { context } => {
                                        let unexpected_no_content = flexpiler::error::source::common::UnexpectedNoContent {
                                            definition_expected: <#struct_intermediary_field_type_vec_ref as flexpiler::identity::Trait>::definition(),
                                        };
                                        let error_source_common: flexpiler::error::source::Common = unexpected_no_content.into();
                                        let error = flexpiler::Error::gen(error_source_common)
                                            .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::FieldTrait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context_field(#struct_intermediary_field_string_vec_ref));
                                        return flexpiler::deserializer::Result::Err(error);
                                    },
                                    flexpiler::deserializer::Result::Err(error) => {
                                        let error = error.propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::FieldTrait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context_field(#struct_intermediary_field_string_vec_ref));
                                        return flexpiler::deserializer::Result::Err(error);
                                    }
                                }
                            }

                            )*

                            _ => {
                                let unrecognized_field = flexpiler::common::rustc::error::UnrecognizedFieldDeclaration {
                                    field_declaration_found: field_declaration_string,
                                    field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                                        #(String::from(#struct_intermediary_field_string_vec_ref),
                                        )*
                                    ]),
                                };
                                let error = flexpiler::Error::gen(unrecognized_field)
                                    .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
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
                                        .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                                    return flexpiler::deserializer::Result::Err(error);
                                }
                            }
                        }

                        match context {
                            flexpiler::common::rustc::deserializer::Context::DataEnd => {
                                break;
                            },
                            flexpiler::common::rustc::deserializer::Context::Separator => {
                                // do nothing
                            },
                            _ => {
                                let unexpected_entry_finish_context = flexpiler::common::rustc::error::UnexpectedEntryFinishContext {
                                    entry_declaration: field_declaration_string,
                                    context_expected: flexpiler::error::ExpectedEntries::from(vec![
                                        flexpiler::common::rustc::deserializer::Context::DataEnd,
                                        flexpiler::common::rustc::deserializer::Context::Separator,
                                    ]),
                                    context_found: context,
                                };
                                let error = flexpiler::Error::gen(unexpected_entry_finish_context)
                                    .propagate(<#struct_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#struct_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                                return flexpiler::deserializer::Result::Err(error);
                            }
                        }
                    } // loop

                    return match <#struct_data_context_intermediary_ident_ref as std::convert::TryInto<#struct_definition_ident_ref>>::try_into(struct_context) {
                        Ok(data) => {
                            flexpiler::deserializer::Result::DataFound {
                                context: flexpiler::common::rustc::deserializer::Context::Freestanding,
                                data,
                            }
                        },
                        Err(error) => {
                            flexpiler::deserializer::Result::Err(error)
                        },
                    }
                }
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterEnum<'a>> for DeserializerTraitImpl {
    fn gen(parameter: ParameterEnum) -> proc_macro2::TokenStream {
        let generics = {
            let mut generic_vec = Vec::new();
            for variable_argumented_ref in parameter.enum_intermediary_ref.variant_argumented_vec.iter() {
                for type_ref in variable_argumented_ref.field_type_ref_vec.iter() {
                    generic_vec.push(quote!{ #type_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format>, });
                }
            }
            for variable_complex_ref in parameter.enum_intermediary_ref.variant_complex_vec.iter() {
                for type_ref in variable_complex_ref.field_type_ref_vec.iter() {
                    generic_vec.push(quote!{ #type_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format>, });
                }
            }
            if generic_vec.is_empty() {
                quote!{}
            } else {
                quote!{
                    where #(#generic_vec)*
                }
            }
        };

        let enum_definition_ident_ref = &parameter.enum_definition_ref.ident;
        let enum_intermediary_ident_string_ref = &parameter.enum_intermediary_ref.ident_string;
        let enum_deserializer_intermediary_ident_ref = &parameter.enum_deserializer_intermediary_ref.ident;
        let enum_variant_ident_string_ref_vec = {
            let mut vec = Vec::new();
            for variant_standalone_ref in parameter.enum_intermediary_ref.variant_standalone_vec.iter() {
                vec.push(&variant_standalone_ref.full_ident_string)
            }
            for variant_argumented_ref in parameter.enum_intermediary_ref.variant_argumented_vec.iter() {
                vec.push(&variant_argumented_ref.full_ident_string)
            }
            for variant_complex_ref in parameter.enum_intermediary_ref.variant_standalone_vec.iter() {
                vec.push(&variant_complex_ref.full_ident_string)
            }
            vec
        };
        //let enum_data_context_intermediary_ident_ref = &parameter.enum_data_context_intermediary_ref.ident;
        //let enum_data_context_intermediary_field_ident_vec_ref = &parameter.enum_data_context_intermediary_ref.field_ident_vec;

        let enum_variant_standalone_deserialization_code_vec = {
            let mut vec = Vec::new();
            for variant_standalone_ref in parameter.enum_intermediary_ref.variant_standalone_vec.iter() {
                let variant_standalone_full_ident_ref = &variant_standalone_ref.full_ident_tokenstream;
                let variant_standalone_full_ident_string_ref = &variant_standalone_ref.full_ident_string;

                vec.push(quote!{
                    #variant_standalone_full_ident_string_ref => {
                        let context: flexpiler::common::rustc::deserializer::Context = declaration_result.finish.into();
                        flexpiler::deserializer::Result::DataFound {
                            data: #variant_standalone_full_ident_ref,
                            context
                        }
                    }
                });
            }
            vec
        };

        let enum_variant_argumented_deserialization_code_vec = {
            let mut vec = Vec::new();
            for variant_argumented_ref in parameter.enum_intermediary_ref.variant_argumented_vec.iter() {
                let variant_argumented_ident_string_ref = &variant_argumented_ref.ident_string;
                let variant_argumented_full_ident_string_ref = &variant_argumented_ref.full_ident_string;

                let deserialization_code = DataArgumentedDeserialization::gen(data_argumented_deserialization::ParameterEnumVariableArgumented{
                    enum_definition_ref: parameter.enum_definition_ref,
                    enum_deserializer_intermediary_ref: &parameter.enum_deserializer_intermediary_ref,
                    variant_argumented_intermediary_ref: variant_argumented_ref,
                });

                vec.push(quote!{
                    #variant_argumented_full_ident_string_ref => {
                        let context: flexpiler::common::rustc::deserializer::Context = declaration_result.finish.into();
                        match context {
                            flexpiler::common::rustc::deserializer::Context::Freestanding => {
                                match flexpiler::parser::parse::<
                                    flexpiler::common::rustc::block::ArgumentStart,
                                    ReaderType
                                >(reader_mut_ref) {
                                    Ok(result) => {},
                                    Err(parser_error) => {
                                        let error = flexpiler::Error::gen(parser_error)
                                            .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_definition_ident_ref, flexpiler::common::rustc::Format>>::context_variant(#variant_argumented_ident_string_ref));
                                        return flexpiler::deserializer::Result::Err(error);
                                    },
                                }
                            },
                            flexpiler::common::rustc::deserializer::Context::ArgumentStart => {
                            },
                            _ => {
                                let incompatible_enum_data_type = flexpiler::common::rustc::error::IncompatibleEnumDataType {
                                    enum_declaration_found: declaration_result.identifier_string,
                                    enum_data_type_expected: flexpiler::common::rustc::error::incompatibleenumdataType::EnumDataType::Argument,
                                    context_found: context,
                                };
                                let error = flexpiler::Error::gen(incompatible_enum_data_type)
                                    .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_definition_ident_ref, flexpiler::common::rustc::Format>>::context_variant(#variant_argumented_ident_string_ref));
                                return flexpiler::deserializer::Result::Err(error);
                            }
                        }

                        #deserialization_code
                    }
                });
            }
            vec
        };

        let enum_variant_complex_deserialization_code_vec = {
            let mut vec = Vec::new();
            for (variant_complex_ref, variant_complex_data_context_intermediary_ref) in parameter.enum_intermediary_ref.variant_complex_vec.iter()
                .zip(parameter.enum_variable_complex_data_context_intermediary_vec_ref.iter()){
                let variant_complex_full_ident_string_ref = &variant_complex_ref.full_ident_string;

                let deserialization_code = DataContextDeserialization::gen(data_context_deserialization::ParameterEnumVariableComplex{
                    enum_definition_ref: parameter.enum_definition_ref,
                    enum_deserializer_intermediary_ref: &parameter.enum_deserializer_intermediary_ref,
                    variable_complex_intermediary_ref: variant_complex_ref,
                    data_context_intermediary_ref: variant_complex_data_context_intermediary_ref
                });

                vec.push(quote!{
                    #variant_complex_full_ident_string_ref => {
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
                                            .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_definition_ident_ref, flexpiler::common::rustc::Format>>::context_variant(#variant_complex_full_ident_string_ref));
                                        return flexpiler::deserializer::Result::Err(error);
                                    },
                                }
                            },
                            flexpiler::common::rustc::deserializer::Context::DataStart => {
                            },
                            _ => {
                                let incompatible_enum_data_type = flexpiler::common::rustc::error::IncompatibleEnumDataType {
                                    enum_declaration_found: declaration_result.identifier_string,
                                    enum_data_type_expected: flexpiler::common::rustc::error::incompatibleenumdataType::EnumDataType::Complex,
                                    context_found: context,
                                };
                                let error = flexpiler::Error::gen(incompatible_enum_data_type)
                                    .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_definition_ident_ref, flexpiler::common::rustc::Format>>::context_variant(#variant_complex_full_ident_string_ref));
                                return flexpiler::deserializer::Result::Err(error);
                            }
                        }

                        #deserialization_code
                    }
                });
            }
            vec
        };

        quote!{
            impl flexpiler::deserializer::Trait<#enum_definition_ident_ref, flexpiler::common::rustc::Format> for #enum_deserializer_intermediary_ident_ref
            #generics {
                fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
                    -> flexpiler::deserializer::Result<#enum_definition_ident_ref, flexpiler::common::rustc::deserializer::Context, flexpiler::Error<flexpiler::common::rustc::error::Source>>
                where ReaderType: flexpiler::reader::Trait {
                    use std::convert::TryInto;
                    use flexpiler::Deserialization;
                    use flexpiler::deserializer::Trait as DeserializerTrait;
                    use flexpiler::deserializer::context::VariantTrait as DeserializerContextVariantTrait;
                    use flexpiler::error::Trait as ErrorTrait;
                    use flexpiler::error::propagation::Trait as ErrorPropagationTrait;
                    use flexpiler::parser::Parse;

                    let declaration_result = match flexpiler::common::rustc::block::IdentifierWithVariableFinish::parse(reader_mut_ref) {
                        Err(parser_error) => {
                            let error = flexpiler::Error::gen(parser_error)
                                .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#enum_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                            return flexpiler::deserializer::Result::Err(error);
                        },
                        Ok(result) => result,
                    };

                    let result = match declaration_result.identifier_string.as_str() {
                        #(#enum_variant_standalone_deserialization_code_vec
                        )*
                        #(#enum_variant_argumented_deserialization_code_vec
                        )*
                        #(#enum_variant_complex_deserialization_code_vec
                        )*

                        _ => {
                            let incompatible_enum_declaration = flexpiler::common::rustc::error::IncompatibleEnumDeclaration {
                                enum_declaration_found: declaration_result.identifier_string,
                                enum_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                                    #(std::string::String::from(#enum_variant_ident_string_ref_vec),)*
                                ]),
                            };
                            let error = flexpiler::Error::gen(incompatible_enum_declaration)
                                .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::Trait<#enum_definition_ident_ref, flexpiler::common::rustc::Format>>::context());
                            return flexpiler::deserializer::Result::Err(error);
                        }
                    };

                    return result;
                }
            }
        }
    }
}
