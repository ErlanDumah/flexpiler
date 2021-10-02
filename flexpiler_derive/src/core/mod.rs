pub mod definition;
pub mod error;
pub use error::Error;
pub mod implementation;
pub mod intermediary;


pub fn flexpiler(ast: syn::DeriveInput) -> proc_macro::TokenStream
{
    use implementation::TryTrait;

    use syn::spanned::Spanned;

    match ast.data {
        syn::Data::Struct(data_struct) => {
            let ident = ast.ident;
            let generics = ast.generics;
            //flexpiler_struct(ident, data_struct)
            let struct_definition = definition::Struct {
                ident,
                data_struct,
                generics,
            };

            let result = match implementation::Flexpiler::try_gen(&struct_definition) {
                Ok(result) => result,
                Err(error) => error.into(),
            };

            println!("flexpiler Output:\n{}", result);

            result.into()
        },

        syn::Data::Enum(data_enum) => {
            let ident = ast.ident;
            let generics = ast.generics;
            //flexpiler_struct(ident, data_struct)
            let enum_definition = definition::Enum {
                ident,
                data_enum,
                generics,
            };

            let result = match implementation::Flexpiler::try_gen(&enum_definition) {
                Ok(result) => result,
                Err(error) => error.into(),
            };

            println!("flexpiler Output:\n{}", result);

            result.into()
        },

        syn::Data::Union(_) => {
            let output = quote_spanned! {
                ast.span() =>
                compile_error!("flexpiler currently does not support unions.");
            };
            output.into()
        },
    }
}
