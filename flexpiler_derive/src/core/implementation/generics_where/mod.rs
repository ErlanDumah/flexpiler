use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct GenericsWhere;


pub struct ParameterGenerics<'a> {
    pub generics_ref: &'a syn::Generics,
}


pub struct ParameterEnum<'a> {
    pub definition_enum_ref: &'a definition::Enum,
    pub intermediary_enum_ref: &'a intermediary::Enum<'a>,
}


pub struct ParameterStruct<'a> {
    pub definition_struct_ref: &'a definition::Struct,
    pub intermediary_struct_ref: &'a intermediary::Struct<'a>,
}


fn pack_generics_into(generics_ref: &syn::Generics, tokenstream_vec_mut_ref: &mut Vec<proc_macro2::TokenStream>) {
    for generic_param_ref in generics_ref.params.iter() {
        match generic_param_ref {
            &syn::GenericParam::Type(ref type_param_ref) => {
                tokenstream_vec_mut_ref.push(quote!{ #type_param_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format> + flexpiler::identity::Trait, });
            },
            &syn::GenericParam::Lifetime(..) => {
                // lifetimes are not important for the where clause
            },
            &syn::GenericParam::Const(..) => {
                // consts are not important for the where clause
            },
        }
    }
}


impl implementation::Trait<&syn::Generics> for GenericsWhere {
    fn gen(generics_ref: &syn::Generics) -> proc_macro2::TokenStream {
        let mut tokenstream_vec = Vec::new();
        pack_generics_into(generics_ref, &mut tokenstream_vec);
        if tokenstream_vec.is_empty() {
            quote!{}
        } else {
            quote!{
                where #(#tokenstream_vec)*
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterEnum<'a>> for GenericsWhere {
    fn gen(parameter: ParameterEnum<'a>) -> proc_macro2::TokenStream {
        let mut tokenstream_vec = Vec::new();
        pack_generics_into(&parameter.definition_enum_ref.generics, &mut tokenstream_vec);
        for variable_argumented_ref in parameter.intermediary_enum_ref.variant_argumented_vec.iter() {
            for type_ref in variable_argumented_ref.field_type_ref_vec.iter() {
                tokenstream_vec.push(quote!{ #type_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format> + flexpiler::identity::Trait, });
            }
        }
        for variable_complex_ref in parameter.intermediary_enum_ref.variant_complex_vec.iter() {
            for type_ref in variable_complex_ref.field_type_ref_vec.iter() {
                tokenstream_vec.push(quote!{ #type_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format> + flexpiler::identity::Trait, });
            }
        }
        if tokenstream_vec.is_empty() {
            quote!{}
        } else {
            quote!{
                where #(#tokenstream_vec)*
            }
        }
    }
}


impl<'a> implementation::Trait<ParameterStruct<'a>> for GenericsWhere {
    fn gen(parameter: ParameterStruct<'a>) -> proc_macro2::TokenStream {
        let mut tokenstream_vec = Vec::new();
        pack_generics_into(&parameter.definition_struct_ref.generics, &mut tokenstream_vec);
        for type_ref in parameter.intermediary_struct_ref.field_type_vec.iter() {
            tokenstream_vec.push(quote!{ #type_ref: flexpiler::Deserialization<flexpiler::common::rustc::Format> + flexpiler::identity::Trait, });
        }
        if tokenstream_vec.is_empty() {
            quote!{}
        } else {
            quote!{
                where #(#tokenstream_vec)*
            }
        }
    }
}
