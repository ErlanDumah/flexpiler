use crate::core::definition;
use crate::core::intermediary;
use crate::core::error;

pub struct DataContext {
    pub ident: syn::Ident,
    pub field_ident_vec: Vec<syn::Ident>,
    pub field_type_vec: Vec<proc_macro2::TokenStream>,
}


// Compiler needs to know that the Error returned by this lives longer than 'intermediary, namely 'definition.
pub struct ParameterEnumVariantComplex<'definition, 'intermediary> {
    pub enum_definition_ref: &'definition definition::Enum,
    pub variant_complex_enum_intermediary_ref: &'intermediary intermediary::enum_rustc::VariantComplex<'definition>,
}


impl<'a> std::convert::TryFrom<&'a definition::Struct> for DataContext {
    type Error = error::Error<'a>;

    fn try_from(struct_definition_ref: &'a definition::Struct) -> std::result::Result<Self, Self::Error> {
        let mut field_ident_vec = Vec::new();
        let mut field_type_vec = Vec::new();

        for field_ref in struct_definition_ref.data_struct.fields.iter() {
            let field_type_ref = &field_ref.ty;
            let field_ident_ref = match &field_ref.ident {
                &Some(ref value_ref) => value_ref,
                &None => {
                    return Err(
                        Self::Error::StructFieldUnnamed{
                            field_ref,
                        },
                    )
                }
            };
            field_ident_vec.push(format_ident!("{}_option", field_ident_ref));
            field_type_vec.push(quote!(Option< #field_type_ref >));
        }

        Ok(DataContext {
            ident: format_ident!("{}flexpilerContext", struct_definition_ref.ident),
            field_ident_vec,
            field_type_vec,
        })
    }
}


// Compiler needs to know that the Error returned by this lives longer than 'intermediary, namely 'definition.
impl<'definition, 'intermediary> std::convert::TryFrom<ParameterEnumVariantComplex<'definition, 'intermediary>> for DataContext {
    type Error = error::Error<'definition>;

    fn try_from(parameter: ParameterEnumVariantComplex<'definition, 'intermediary>) -> std::result::Result<Self, Self::Error> {
        let mut field_ident_vec = Vec::new();
        let mut field_type_vec = Vec::new();

        for (field_ident_ref, field_type_ref) in parameter.variant_complex_enum_intermediary_ref.field_ident_ref_vec.iter()
            .zip(parameter.variant_complex_enum_intermediary_ref.field_type_ref_vec.iter()) {
            field_ident_vec.push(format_ident!("{}_option", field_ident_ref));
            field_type_vec.push(quote!(Option< #field_type_ref >));
        }

        let enum_ident_ref = &parameter.enum_definition_ref.ident;

        Ok(DataContext {
            ident: format_ident!("{}{}flexpilerContext",
                parameter.enum_definition_ref.ident,
                parameter.variant_complex_enum_intermediary_ref.ident_ref
            ),
            field_ident_vec,
            field_type_vec,
        })
    }
}
