use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct TryIntoResultImpl;


pub struct ParameterStruct<'a> {
    pub struct_definition_ref: &'a definition::Struct,
    pub struct_intermediary_ref: &'a intermediary::Struct<'a>,
    pub data_context_intermediary_ref: &'a intermediary::DataContext,
    pub struct_deserializer_intermediary_ref: &'a intermediary::deserializer::Struct,
}


pub struct ParameterEnum<'a> {
    pub enum_definition_ref: &'a definition::Enum,
    pub enum_variant_complex_intermediary_ref: &'a intermediary::enum_rustc::VariantComplex<'a>,
    pub data_context_intermediary_ref: &'a intermediary::DataContext,
    pub enum_deserializer_intermediary_ref: &'a intermediary::deserializer::Enum,
}


impl<'a> implementation::Trait<ParameterStruct<'a>> for TryIntoResultImpl {
    fn gen(parameter: ParameterStruct) -> proc_macro2::TokenStream {
        let struct_definition_ident_ref = &parameter.struct_definition_ref.ident;
        let struct_intermediary_ident_string_ref = &parameter.struct_intermediary_ref.ident_string;
        let struct_intermediary_field_ident_vec_ref = &parameter.struct_intermediary_ref.field_ident_vec;
        let struct_intermediary_field_string_vec_ref = &parameter.struct_intermediary_ref.field_string_vec;
        let struct_data_context_intermediary_ident_ref = &parameter.data_context_intermediary_ref.ident;
        let struct_data_context_intermediary_field_ident_vec_ref = &parameter.data_context_intermediary_ref.field_ident_vec;
        let struct_deserializer_intermediary_ident_ref = &parameter.struct_deserializer_intermediary_ref.ident;

        quote!{
            impl std::convert::TryInto<#struct_definition_ident_ref> for #struct_data_context_intermediary_ident_ref {
                type Error = flexpiler::Error;

                fn try_into(self) -> std::result::Result<#struct_definition_ident_ref, flexpiler::Error> {
                    use flexpiler::error::Trait as ErrorTrait;

                    #(let #struct_intermediary_field_ident_vec_ref = match self.#struct_data_context_intermediary_field_ident_vec_ref {
                        Some(value) => value,
                        None => {
                            let missing_struct_field = flexpiler::error::MissingStructField {
                                struct_declaration_found: std::string::String::from(#struct_intermediary_ident_string_ref),
                                field_declaration_expected: std::string::String::from(#struct_intermediary_field_string_vec_ref),
                            };
                            let error = flexpiler::Error::gen(missing_struct_field)
                                .propagate(#struct_deserializer_intermediary_ident_ref::context_general());
                            return Err(error);
                        }
                    };
                    )*

                    Ok(#struct_definition_ident_ref {
                        #(#struct_intermediary_field_ident_vec_ref,
                        )*
                    })
                }
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterEnum<'a>> for TryIntoResultImpl {
    fn gen(parameter: ParameterEnum) -> proc_macro2::TokenStream {
        let enum_definition_ident_ref = &parameter.enum_definition_ref.ident;
        let enum_variant_complex_full_ident_ref = &parameter.enum_variant_complex_intermediary_ref.full_ident_tokenstream;
        let enum_variant_complex_full_ident_string_ref = &parameter.enum_variant_complex_intermediary_ref.full_ident_string;
        let enum_variant_complex_intermediary_field_ident_ref_vec_ref = &parameter.enum_variant_complex_intermediary_ref.field_ident_ref_vec;
        let enum_variant_complex_intermediary_field_string_vec_ref = &parameter.enum_variant_complex_intermediary_ref.field_string_vec;
        let enum_variant_data_context_intermediary_ident_ref = &parameter.data_context_intermediary_ref.ident;
        let enum_variant_data_context_intermediary_field_ident_vec_ref = &parameter.data_context_intermediary_ref.field_ident_vec;
        let enum_deserializer_intermediary_ident_ref = &parameter.enum_deserializer_intermediary_ref.ident;

        quote!{
            impl std::convert::TryInto<#enum_definition_ident_ref> for #enum_variant_data_context_intermediary_ident_ref {
                type Error = flexpiler::Error;

                fn try_into(self) -> std::result::Result<#enum_definition_ident_ref, flexpiler::Error> {
                    use flexpiler::error::Trait as ErrorTrait;

                    #(let #enum_variant_complex_intermediary_field_ident_ref_vec_ref = match self.#enum_variant_data_context_intermediary_field_ident_vec_ref {
                        Some(value) => value,
                        None => {
                            let missing_enum_field = flexpiler::error::MissingEnumComplexField {
                                enum_declaration_found: std::string::String::from(#enum_variant_complex_full_ident_string_ref),
                                field_declaration_expected: std::string::String::from(#enum_variant_complex_intermediary_field_string_vec_ref),
                            };
                            let error = flexpiler::Error::gen(missing_enum_field)
                                .propagate(#enum_deserializer_intermediary_ident_ref::context_entry_general(#enum_variant_complex_full_ident_string_ref));
                            return Err(error);
                        }
                    };
                    )*

                    Ok(#enum_variant_complex_full_ident_ref {
                        #(#enum_variant_complex_intermediary_field_ident_ref_vec_ref,
                        )*
                    })
                }
            }
        }
    }
}
