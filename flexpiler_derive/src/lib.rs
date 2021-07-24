// extern - std
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

// extern - intern
extern crate flexpiler;

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


/*
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn simple_derive() {
        let input_stream = proc_macro::TokenStream::from_str(
            "struct Test {\
                pub a_string: String,
                pub a_int32: i32,
                pub a_usize: usize,
            }
            "
        ).unwrap();

        let output_stream = crate::flexpiler_test(input_stream);

        println!("{}", output_stream.to_string());

        assert!(false, "Debug test");
    }
}
*/
