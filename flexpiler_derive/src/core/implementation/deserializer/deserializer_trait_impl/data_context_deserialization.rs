use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct DataContextDeserialization;


pub struct ParameterEnumVariableComplex<'a> {
    pub enum_definition_ref: &'a definition::Enum,
    pub data_context_intermediary_ref: &'a intermediary::DataContext<'a>,
    pub variable_complex_intermediary_ref: &'a intermediary::enum_rustc::VariantComplex<'a>,
    pub enum_deserializer_intermediary_ref: &'a intermediary::deserializer::Enum,
}


impl<'a> implementation::Trait<ParameterEnumVariableComplex<'a>> for DataContextDeserialization {
    fn gen(parameter: ParameterEnumVariableComplex<'a>) -> proc_macro2::TokenStream {

        let enum_ident_ref = &parameter.enum_definition_ref.ident;
        let enum_generics_ref = &parameter.enum_definition_ref.generics;
        let data_context_intermediary_ident_ref = &parameter.data_context_intermediary_ref.ident;
        let data_context_intermediary_generics_ref = &parameter.data_context_intermediary_ref.generics_ref;
        let data_context_field_ident_vec_ref = &parameter.data_context_intermediary_ref.field_ident_vec;
        let variable_complex_intermediary_ident_string_ref = &parameter.variable_complex_intermediary_ref.full_ident_string;
        let variable_complex_intermediary_full_ident_string_ref = &parameter.variable_complex_intermediary_ref.full_ident_string;
        let variable_complex_intermediary_field_ident_ref_vec_ref = &parameter.variable_complex_intermediary_ref.field_ident_ref_vec;
        let variable_complex_intermediary_field_ident_string_vec_ref = &parameter.variable_complex_intermediary_ref.field_string_vec;
        let variable_complex_intermediary_field_type_ref_vec_ref = &parameter.variable_complex_intermediary_ref.field_type_ref_vec;
        let enum_deserializer_intermediary_ident_ref = &parameter.enum_deserializer_intermediary_ref.ident;

        quote!{
            let mut data_context = #data_context_intermediary_ident_ref::default();

            loop {
                let field_declaration_string = match flexpiler::common::rustc::block::DeclarationOrDataEnd::parse(reader_mut_ref) {
                    Err(parser_error) => {
                        let error = flexpiler::error::Error::gen(parser_error)
                            .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_ident_ref #enum_generics_ref, flexpiler::common::rustc::Format>>::context_variant(#variable_complex_intermediary_ident_string_ref));
                        return flexpiler::deserializer::Result::Err(error);
                    },
                    Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::DataEnd()) => {
                        break;
                    },
                    Ok(flexpiler::common::rustc::block::declaration_or_data_end::Result::Declaration(declaration)) => {
                        declaration
                    },
                };

                let context = match field_declaration_string.as_str() {
                    #(#variable_complex_intermediary_field_ident_string_vec_ref => {
                        let result = <#variable_complex_intermediary_field_type_ref_vec_ref as flexpiler::Deserialization<flexpiler::common::rustc::Format>>
                                    ::Deserializer::deserialize(reader_mut_ref);
                        match result {
                            flexpiler::deserializer::Result::DataFound{ data, context } => {
                                data_context.#data_context_field_ident_vec_ref = Some(data);
                                context
                            },
                            flexpiler::deserializer::Result::NoDataFound{ .. } => {
                                let unexpected_no_content = flexpiler::error::source::common::UnexpectedNoContent {
                                    definition_expected: <#variable_complex_intermediary_field_type_ref_vec_ref as flexpiler::identity::Trait>::definition(),
                                };
                                let error_source_common: flexpiler::error::source::Common = unexpected_no_content.into();
                                let error = flexpiler::Error::gen(error_source_common)
                                    .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantFieldTrait<#enum_ident_ref #enum_generics_ref, flexpiler::common::rustc::Format>>::context_variant_field(#variable_complex_intermediary_full_ident_string_ref, #variable_complex_intermediary_field_ident_string_vec_ref));
                                return flexpiler::deserializer::Result::Err(error);
                            },
                            flexpiler::deserializer::Result::Err(deserializer_error) => {
                                let error = deserializer_error
                                    .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantFieldTrait<#enum_ident_ref #enum_generics_ref, flexpiler::common::rustc::Format>>::context_variant_field(#variable_complex_intermediary_full_ident_string_ref, #variable_complex_intermediary_field_ident_string_vec_ref));
                                return flexpiler::deserializer::Result::Err(error);
                            },
                        }
                    }

                    )*

                    _ => {
                        let unrecognized_field = flexpiler::common::rustc::error::UnrecognizedFieldDeclaration {
                            field_declaration_found: field_declaration_string,
                            field_declaration_expected_entries: flexpiler::error::ExpectedEntries::from(vec![
                                #(String::from(#variable_complex_intermediary_field_ident_string_vec_ref),
                                )*
                            ]),
                        };
                        let error = flexpiler::Error::gen(unrecognized_field)
                            .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_ident_ref #enum_generics_ref, flexpiler::common::rustc::Format>>::context_variant(#variable_complex_intermediary_ident_string_ref));
                        return flexpiler::deserializer::Result::Err(error);
                    }
                };

                match context {
                    flexpiler::common::rustc::deserializer::Context::Freestanding => {
                        match flexpiler::common::rustc::block::DataEndOrSeparator::parse(reader_mut_ref) {
                            Ok(flexpiler::common::rustc::block::data_end_or_separator::Result::DataEnd) => {
                                break;
                            },
                            Ok(flexpiler::common::rustc::block::data_end_or_separator::Result::Separator) => {
                                // do nothing
                            },
                            Err(parser_error) => {
                                let error = flexpiler::error::Error::gen(parser_error)
                                    .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_ident_ref #enum_generics_ref, flexpiler::common::rustc::Format>>::context_variant(#variable_complex_intermediary_ident_string_ref));
                                return flexpiler::deserializer::Result::Err(error);
                            },
                        }
                    },
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
                            .propagate(<#enum_deserializer_intermediary_ident_ref as flexpiler::deserializer::context::VariantTrait<#enum_ident_ref #enum_generics_ref, flexpiler::common::rustc::Format>>::context_variant(#variable_complex_intermediary_ident_string_ref));
                        return flexpiler::deserializer::Result::Err(error);
                    }
                }
            } // loop

            match <#data_context_intermediary_ident_ref #data_context_intermediary_generics_ref as std::convert::TryInto<#enum_ident_ref #enum_generics_ref>>::try_into(data_context) {
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
