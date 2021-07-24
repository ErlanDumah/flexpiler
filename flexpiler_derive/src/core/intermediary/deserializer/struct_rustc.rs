use crate::core::definition;
use crate::core::implementation;


pub struct Struct {
    pub ident: syn::Ident,
}


impl From<&definition::Struct> for Struct {
    fn from(struct_definition_ref: &definition::Struct) -> Self {
        Struct {
            ident: format_ident!("{}flexpilerDeserializer", struct_definition_ref.ident),
        }
    }
}
