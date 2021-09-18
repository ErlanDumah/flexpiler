use crate::core::definition;
use crate::core::error;
use std::env::var;


pub struct VariantStandalone<'a> {
    pub ident_ref: &'a syn::Ident,
    pub ident_string: std::string::String,
    pub full_ident_tokenstream: proc_macro2::TokenStream,
    pub full_ident_string: std::string::String,
}

pub struct VariantArgumented<'a> {
    pub ident_ref: &'a syn::Ident,
    pub ident_string: std::string::String,
    pub full_ident_tokenstream: proc_macro2::TokenStream,
    pub full_ident_string: std::string::String,
    pub field_type_ref_vec: Vec<&'a syn::Type>,
}

pub struct VariantComplex<'a> {
    pub ident_ref: &'a syn::Ident,
    pub ident_string: std::string::String,
    pub full_ident_tokenstream: proc_macro2::TokenStream,
    pub full_ident_string: std::string::String,
    pub field_ident_ref_vec: Vec<&'a syn::Ident>,
    pub field_string_vec: Vec<std::string::String>,
    pub field_type_ref_vec: Vec<&'a syn::Type>,
}


pub struct Enum<'a> {
    pub ident_string: std::string::String,
    pub variant_standalone_vec: Vec<VariantStandalone<'a>>,
    pub variant_argumented_vec: Vec<VariantArgumented<'a>>,
    pub variant_complex_vec: Vec<VariantComplex<'a>>,
}


impl<'a> std::convert::TryFrom<&'a definition::Enum> for Enum<'a> {
    type Error = error::Error<'a>;

    fn try_from(enum_definition_ref: &'a definition::Enum) -> std::result::Result<Self, Self::Error> {

        let mut variant_standalone_vec = Vec::new();
        let mut variant_argumented_vec = Vec::new();
        let mut variant_complex_vec = Vec::new();

        for variant_ref in enum_definition_ref.data_enum.variants.iter() {
            let ident_ref = &variant_ref.ident;
            let ident_string = format!("{}", variant_ref.ident);
            let enum_definition_ident_ref = &enum_definition_ref.ident;
            let variant_ident_ref = &variant_ref.ident;
            let full_ident_tokenstream = quote!(#enum_definition_ident_ref::#variant_ident_ref);
            let full_ident_string = format!("{}::{}",
                enum_definition_ref.ident,
                variant_ref.ident,
            );
            match &variant_ref.fields {
                &syn::Fields::Unit => {
                    variant_standalone_vec.push(VariantStandalone{
                        ident_ref,
                        ident_string,
                        full_ident_tokenstream,
                        full_ident_string,
                    });
                },
                &syn::Fields::Unnamed(ref fields_unnamed_ref) => {
                    let mut field_type_ref_vec = Vec::new();
                    for field_ref in fields_unnamed_ref.unnamed.iter() {
                        field_type_ref_vec.push(&field_ref.ty);
                    }

                    variant_argumented_vec.push(VariantArgumented{
                        ident_ref,
                        ident_string,
                        full_ident_tokenstream,
                        full_ident_string,
                        field_type_ref_vec,
                    })
                }
                &syn::Fields::Named(ref fields_named_ref) => {
                    let mut field_ident_ref_vec = Vec::new();
                    let mut field_string_vec = Vec::new();
                    let mut field_type_ref_vec = Vec::new();
                    for field_ref in fields_named_ref.named.iter() {
                        let field_ident_ref = match field_ref.ident {
                            Some(ref ident_ref) => ident_ref,
                            None => {
                                return Err(error::Error::StructFieldUnnamed {
                                    field_ref,
                                });
                            }
                        };

                        field_ident_ref_vec.push(field_ident_ref);
                        field_string_vec.push(format!("{}", field_ident_ref));
                        field_type_ref_vec.push(&field_ref.ty);
                    }

                    variant_complex_vec.push(VariantComplex{
                        ident_ref,
                        ident_string,
                        full_ident_tokenstream,
                        full_ident_string,
                        field_ident_ref_vec,
                        field_string_vec,
                        field_type_ref_vec,
                    })
                }
            }
        }

        Ok(Enum{
            ident_string: format!("{}", enum_definition_ref.ident),
            variant_standalone_vec,
            variant_argumented_vec,
            variant_complex_vec,
        })
    }
}
