use crate::core::implementation;
use crate::core::intermediary;


pub struct ErrorContext;


pub struct ParameterEnum<'a> {
    pub enum_intermediary_ref: &'a intermediary::Enum<'a>,
    pub enum_deserializer_intermediary_ref: &'a intermediary::deserializer::Enum,
}


pub struct ParameterStruct<'a> {
    pub struct_intermediary_ref: &'a intermediary::Struct<'a>,
    pub struct_deserializer_intermediary_ref: &'a intermediary::deserializer::Struct,
}


impl<'a> implementation::Trait<ParameterEnum<'a>> for ErrorContext {
    fn gen(parameter: ParameterEnum) -> proc_macro2::TokenStream {
        let enum_intermediary_ident_string_ref = &parameter.enum_intermediary_ref.ident_string;
        let enum_deserializer_intermediary_ident_ref = &parameter.enum_deserializer_intermediary_ref.ident;

        quote!{
            impl #enum_deserializer_intermediary_ident_ref {
                fn context_general() -> flexpiler::error::Context {
                    flexpiler::error::Context {
                        trace: std::string::String::from(#enum_intermediary_ident_string_ref),
                    }
                }
                fn context_entry_field(enum_full_name: &str,
                           field_name: &str) -> flexpiler::error::Context {
                    let mut trace = std::string::String::from(enum_full_name);
                    trace.push_str("[");
                    trace.push_str(field_name);
                    trace.push_str("]");
                    flexpiler::error::Context {
                        trace,
                    }
                }
                fn context_entry_general(enum_full_name: &str) -> flexpiler::error::Context {
                    let mut trace = std::string::String::from(enum_full_name);
                    flexpiler::error::Context {
                        trace,
                    }
                }
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterStruct<'a>> for ErrorContext {
    fn gen(parameter: ParameterStruct) -> proc_macro2::TokenStream {
        let struct_intermediary_ident_string_ref = &parameter.struct_intermediary_ref.ident_string;
        let struct_deserializer_intermediary_ident_ref = &parameter.struct_deserializer_intermediary_ref.ident;
        
        quote!{
            impl #struct_deserializer_intermediary_ident_ref {
                fn context_field(field_name: &str) -> flexpiler::error::Context {
                    let mut trace = std::string::String::from(#struct_intermediary_ident_string_ref);
                    trace.push_str("[");
                    trace.push_str(field_name);
                    trace.push_str("]");
                    flexpiler::error::Context {
                        trace,
                    }
                }

                fn context_general() -> flexpiler::error::Context {
                    flexpiler::error::Context {
                        trace: std::string::String::from(#struct_intermediary_ident_string_ref),
                    }
                }
            }
        }
    }
}