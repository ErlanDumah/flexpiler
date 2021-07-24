pub mod default_impl;
pub use default_impl::DefaultImpl;
pub mod try_into_result_impl;
pub use try_into_result_impl::TryIntoResultImpl;


use crate::core::definition;
use crate::core::implementation;
use crate::core::intermediary;


pub struct DataContext;


impl implementation::Trait<&intermediary::DataContext> for DataContext {
    fn gen(data_context_ref: &intermediary::DataContext) -> proc_macro2::TokenStream {
        let data_context_ident_ref = &data_context_ref.ident;
        let data_context_field_ident_vec_ref = &data_context_ref.field_ident_vec;
        let data_context_field_type_vec_ref = &data_context_ref.field_type_vec;

        quote!{
            pub struct #data_context_ident_ref {
                #(#data_context_field_ident_vec_ref: #data_context_field_type_vec_ref,
                )*
            }
        }
    }
}
