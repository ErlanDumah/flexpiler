use crate::core::implementation;
use crate::core::intermediary;


pub struct DefaultImpl;


impl<'a> implementation::Trait<&'a intermediary::DataContext<'a>> for DefaultImpl {
    fn gen(data_context_ref: &'a intermediary::DataContext<'a>) -> proc_macro2::TokenStream {
        let data_context_ident_ref = &data_context_ref.ident;
        let data_context_generics_ref = data_context_ref.generics_ref;
        let data_context_field_ident_vec_ref = &data_context_ref.field_ident_vec;

        quote!{
            impl #data_context_generics_ref Default for #data_context_ident_ref #data_context_generics_ref {
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
