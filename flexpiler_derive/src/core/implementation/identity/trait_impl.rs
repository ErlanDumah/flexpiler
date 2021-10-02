use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct TraitImpl;


pub struct ParameterEnum<'a> {
    pub definition_enum_ref: &'a definition::Enum,
    pub intermediary_enum_ref: &'a intermediary::Enum<'a>,
}


pub struct ParameterStruct<'a> {
    pub definition_struct_ref: &'a definition::Struct,
    pub intermediary_struct_ref: &'a intermediary::Struct<'a>,
}


impl<'a> implementation::Trait<ParameterEnum<'a>> for TraitImpl {
    fn gen(parameter: ParameterEnum<'a>) -> proc_macro2::TokenStream {

        let definition_enum_ident_ref = &parameter.definition_enum_ref.ident;
        let definition_enum_generics_ref = &parameter.definition_enum_ref.generics;
        let intermediary_enum_ident_string_ref = &parameter.intermediary_enum_ref.ident_string;

        quote!{
            impl #definition_enum_generics_ref flexpiler::identity::Trait for #definition_enum_ident_ref #definition_enum_generics_ref {
                fn definition() -> String {
                    return std::string::String::from(#intermediary_enum_ident_string_ref);
                }
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterStruct<'a>> for TraitImpl {
    fn gen(parameter: ParameterStruct<'a>) -> proc_macro2::TokenStream {

        let definition_struct_ident_ref = &parameter.definition_struct_ref.ident;
        let definition_struct_generics_ref = &parameter.definition_struct_ref.generics;
        let intermediary_struct_ident_string_ref = &parameter.intermediary_struct_ref.ident_string;

        quote!{
            impl #definition_struct_generics_ref flexpiler::identity::Trait for #definition_struct_ident_ref #definition_struct_generics_ref {
                fn definition() -> String {
                    return std::string::String::from(#intermediary_struct_ident_string_ref);
                }
            }
        }
    }
}
