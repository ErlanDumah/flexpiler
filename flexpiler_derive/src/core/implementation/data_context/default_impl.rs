use crate::core::implementation;
use crate::core::intermediary;


pub struct DefaultImpl;


impl implementation::Trait<&intermediary::DataContext> for DefaultImpl {
    fn gen(data_context_ref: &intermediary::DataContext) -> proc_macro2::TokenStream {
        let data_context_ident_ref = &data_context_ref.ident;
        let data_context_field_ident_vec_ref = &data_context_ref.field_ident_vec;

        quote!{
            impl Default for #data_context_ident_ref {
                fn default() -> Self {
                    #data_context_ident_ref {
                        #(#data_context_field_ident_vec_ref: None,
                        )*
                    }
                }
            }
        }
    }
}
