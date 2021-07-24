

pub enum Error<'a> {
    StructFieldUnnamed{
        field_ref: &'a syn::Field,
    },
}


impl<'a> Into<proc_macro2::TokenStream> for Error<'a> {
    fn into(self) -> proc_macro2::TokenStream {
        use syn::spanned::Spanned;

        match self {
            Error::StructFieldUnnamed { field_ref } => {
                quote_spanned! {
                    field_ref.span() =>
                    compile_error!("flexpiler relies on field names to specify serialisation. Unnamed fields are unsupported.");
                }
            },
        }
    }
}
