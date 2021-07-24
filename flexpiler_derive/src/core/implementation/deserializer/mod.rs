pub mod error_context;
pub use error_context::ErrorContext;
pub mod deserializer_trait_impl;
pub use deserializer_trait_impl::DeserializerTraitImpl;
use crate::core::implementation;
use crate::core::intermediary;


pub struct Deserializer;


impl implementation::Trait<&intermediary::deserializer::Struct> for Deserializer {
    fn gen(struct_deserializer: &intermediary::deserializer::Struct) -> proc_macro2::TokenStream {
        let struct_deserializer_ident_ref = &struct_deserializer.ident;

        quote!{
            pub struct #struct_deserializer_ident_ref;
        }
    }
}


impl implementation::Trait<&intermediary::deserializer::Enum> for Deserializer {
    fn gen(enum_deserializer_ref: &intermediary::deserializer::Enum) -> proc_macro2::TokenStream {
        let enum_deserializer_ident_ref = &enum_deserializer_ref.ident;

        quote!{
            pub struct #enum_deserializer_ident_ref;
        }
    }
}
