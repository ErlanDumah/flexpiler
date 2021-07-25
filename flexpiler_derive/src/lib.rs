// extern - std
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

// mod - intern
mod core;


#[proc_macro_derive(Deserialize)]
pub fn flexpiler_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    core::flexpiler(ast)
}
