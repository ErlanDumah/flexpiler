use crate::core::definition;
use crate::core::implementation;


pub struct Enum {
    pub ident: syn::Ident,
}


impl From<&definition::Enum> for Enum {
    fn from(enum_definition_ref: &definition::Enum) -> Self {
        Enum {
            ident: format_ident!("{}flexpilerDeserializer", enum_definition_ref.ident),
        }
    }
}
