use crate::core::implementation;
use crate::core::intermediary;


pub struct DataArgumentedDeserialization;


pub struct ParameterEnumVariableArgumented<'a> {
    pub variant_argumented_intermediary_ref: &'a intermediary::enum_rustc::VariantArgumented<'a>,
    pub enum_deserializer_intermediary_ref: &'a intermediary::deserializer::Enum,
}


impl DataArgumentedDeserialization {
    fn gen_separator_catch(data_ident_string_ref: &std::string::String,
                           deserializer_ident_ref: &syn::Ident,
                           argument_result_ident_ref: &syn::Ident)
        -> proc_macro2::TokenStream
    {
        quote!{
            match #argument_result_ident_ref.context {
                flexpiler::common::rustc::deserializer::Context::Separator => {},
                flexpiler::common::rustc::deserializer::Context::Freestanding => {
                    match flexpiler::common::rustc::block::Separator::parse(reader_mut_ref) {
                        Ok(result) => {},
                        Err(parser_error) => {
                            let error = flexpiler::error::Error::gen(parser_error)
                                .propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                            return Err(error);
                        },
                    }
                },
                _ => {
                    let missing_argument_closure = flexpiler::error::MissingEnumArgumentSeparator{
                        enum_declaration_found: declaration_result.identifier_string,
                    };
                    let error = flexpiler::Error::gen(missing_argument_closure)
                        .propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                    return Err(error);
                },
            }
        }
    }

    fn gen_separator_or_argumentend_catch(data_ident_string_ref: &std::string::String,
                                          deserializer_ident_ref: &syn::Ident,
                                          argument_result_ident_ref: &syn::Ident)
                                          -> proc_macro2::TokenStream
    {
        quote!{
            match #argument_result_ident_ref.context {
                flexpiler::common::rustc::deserializer::Context::ArgumentEnd => {},
                flexpiler::common::rustc::deserializer::Context::Separator => {
                    match flexpiler::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                        Ok(_) => {},
                        Err(parser_error) => {
                            let error = flexpiler::error::Error::gen(parser_error)
                                .propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                            return Err(error);
                        },
                    }
                },
                flexpiler::common::rustc::deserializer::Context::Freestanding => {
                    match flexpiler::common::rustc::block::ArgumentEndOrSeparator::parse(reader_mut_ref) {
                        Ok(flexpiler::common::rustc::block::argument_end_or_separator::Result::ArgumentEnd) => {},
                        Ok(flexpiler::common::rustc::block::argument_end_or_separator::Result::Separator) => {
                            match flexpiler::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                                Ok(_) => {},
                                Err(parser_error) => {
                                    let error = flexpiler::error::Error::gen(parser_error)
                                        .propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                                    return Err(error);
                                }
                            }
                        },
                        Err(parser_error) => {
                            let error = flexpiler::error::Error::gen(parser_error)
                                .propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                            return Err(error);
                        },
                    }
                },
                _ => {
                    let missing_argument_closure = flexpiler::error::MissingEnumArgumentSeparator{
                        enum_declaration_found: declaration_result.identifier_string,
                    };
                    let error = flexpiler::Error::gen(missing_argument_closure)
                        .propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                    return Err(error);
                },
            }
        }
    }

    fn gen_argument_deserialization(data_ident_string_ref: &std::string::String,
                                    deserializer_ident_ref: &syn::Ident,
                                    argument_result_ident_ref: &syn::Ident,
                                    field_type_ref: &syn::Type)
        -> proc_macro2::TokenStream
    {
        quote! {
            let #argument_result_ident_ref = match <#field_type_ref as flexpiler::Deserialization<flexpiler::common::rustc::Format>>
                        ::Deserializer::deserialize(reader_mut_ref) {
                Ok(value) => value,
                Err(error) => {
                    let error = error.propagate(#deserializer_ident_ref::context_entry_general(#data_ident_string_ref));
                    return Err(error);
                }
            };
        }
    }
}


impl<'a> implementation::Trait<ParameterEnumVariableArgumented<'a>> for DataArgumentedDeserialization {
    fn gen(parameter: ParameterEnumVariableArgumented<'a>) -> proc_macro2::TokenStream {
        let variable_argumented_intermediary_full_ident_tokenstream_ref = &parameter.variant_argumented_intermediary_ref.full_ident_tokenstream;
        let variable_argumented_intermediary_full_ident_string_ref = &parameter.variant_argumented_intermediary_ref.full_ident_string;
        let variable_argumented_intermediary_field_type_ref_vec_ref = &parameter.variant_argumented_intermediary_ref.field_type_ref_vec;
        let enum_deserializer_intermediary_ident_ref = &parameter.enum_deserializer_intermediary_ref.ident;

        let (argument_ident_vec, argument_deserialization_code_vec) = {
            let mut argument_result_ident_vec = Vec::new();
            let mut argument_deserialization_vec = Vec::new();
            let mut enumerated_field_type_iter = parameter.variant_argumented_intermediary_ref.field_type_ref_vec.iter()
                .enumerate();

            struct Element<'a> {
                pub idx: usize,
                pub type_ref: &'a syn::Type,
            }

            let mut element_option = match enumerated_field_type_iter.next() {
                Some((idx, type_ref)) => Some(Element{
                    idx,
                    type_ref,
                }),
                None => None,
            };
            loop {
                let (idx, type_ref) = match element_option {
                    None => { break; },
                    Some(element) => (element.idx, element.type_ref),
                };

                let argument_result_ident = format_ident!("argument_{}_result", idx);
                let serialization_code = Self::gen_argument_deserialization(
                    &parameter.variant_argumented_intermediary_ref.full_ident_string,
                    &parameter.enum_deserializer_intermediary_ref.ident,
                    &argument_result_ident,
                    type_ref,
                );

                element_option = match enumerated_field_type_iter.next() {
                    Some((idx, type_ref)) => Some(Element{
                        idx,
                        type_ref,
                    }),
                    None => None,
                };

                let catch_code = if element_option.is_none() {
                    Self::gen_separator_or_argumentend_catch(
                        &parameter.variant_argumented_intermediary_ref.full_ident_string,
                        &parameter.enum_deserializer_intermediary_ref.ident,
                        &argument_result_ident,
                    )
                } else {
                    Self::gen_separator_catch(
                        &parameter.variant_argumented_intermediary_ref.full_ident_string,
                        &parameter.enum_deserializer_intermediary_ref.ident,
                        &argument_result_ident,
                    )
                };

                argument_deserialization_vec.push(quote!{
                    #serialization_code

                    #catch_code
                });
                argument_result_ident_vec.push(argument_result_ident);

            }
            (argument_result_ident_vec, argument_deserialization_vec)
        };

        let argument_full_deserialization_code = if argument_deserialization_code_vec.is_empty() {
            quote!{
                match flexpiler::common::rustc::block::ArgumentEnd::parse(reader_mut_ref) {
                    Ok(result) => {},
                    Err(parser_error) => {
                        let error = flexpiler::error::Error::gen(parser_error)
                            .propagate(#enum_deserializer_intermediary_ident_ref::context_entry_general(#variable_argumented_intermediary_full_ident_string_ref));
                        return Err(error);
                    }
                }
            }
        } else {
            quote!{
                #(#argument_deserialization_code_vec
                )*
            }
        };

        quote!{
            #argument_full_deserialization_code

            let data = #variable_argumented_intermediary_full_ident_tokenstream_ref(
                #(#argument_ident_vec.data
                )*
            );

            Ok(flexpiler::deserializer::Result {
                context: flexpiler::common::rustc::deserializer::Context::Freestanding,
                data,
            })
        }
    }
}
