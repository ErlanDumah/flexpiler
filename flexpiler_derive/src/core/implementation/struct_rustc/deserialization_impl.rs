use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct DeserializationImpl;


pub struct ParameterStruct<'a> {
    pub struct_definition_ref: &'a definition::Struct,
    pub struct_deserializer_intermediary_ref: &'a intermediary::deserializer::Struct,
}


pub struct ParameterEnum<'a> {
    pub enum_definition_ref: &'a definition::Enum,
    pub enum_deserializer_intermediary_ref: &'a intermediary::deserializer::Enum,
}


impl<'a> implementation::Trait<ParameterStruct<'a>> for DeserializationImpl {
    fn gen(parameter: ParameterStruct) -> proc_macro2::TokenStream {
        let struct_ident_ref = &parameter.struct_definition_ref.ident;
        let struct_generics_ref = &parameter.struct_definition_ref.generics;
        let struct_deserializer_ident_ref = &parameter.struct_deserializer_intermediary_ref.ident;

        let generics_where = implementation::GenericsWhere::gen(struct_generics_ref);

        quote!{
            impl #struct_generics_ref flexpiler::Deserialization<flexpiler::common::rustc::Format> for #struct_ident_ref #struct_generics_ref
            #generics_where {
                type Deserializer = #struct_deserializer_ident_ref;
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterEnum<'a>> for DeserializationImpl {
    fn gen(parameter: ParameterEnum) -> proc_macro2::TokenStream {
        let enum_ident_ref = &parameter.enum_definition_ref.ident;
        let enum_generics_ref = &parameter.enum_definition_ref.generics;
        let enum_deserializer_ident_ref = &parameter.enum_deserializer_intermediary_ref.ident;

        let generics_where = implementation::GenericsWhere::gen(enum_generics_ref);

        quote!{
            impl #enum_generics_ref flexpiler::Deserialization<flexpiler::common::rustc::Format> for #enum_ident_ref #enum_generics_ref
            #generics_where {
                type Deserializer = #enum_deserializer_ident_ref;
            }
        }
    }
}
