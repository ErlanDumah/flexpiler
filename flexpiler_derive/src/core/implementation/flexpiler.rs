use crate::core::definition;
use crate::core::error;
use crate::core::implementation;
use crate::core::intermediary;
use std::ops::Index;


pub struct Flexpiler;


impl<'a> implementation::TryTrait<&'a definition::Struct> for Flexpiler {
    type Error = error::Error<'a>;

    fn try_gen(struct_definition_ref: &'a definition::Struct) -> std::result::Result<proc_macro2::TokenStream, Self::Error> {
        use std::convert::TryFrom;
        use implementation::Trait;

        let struct_intermediary = match intermediary::Struct::try_from(struct_definition_ref) {
            Ok(struct_intermediary) => struct_intermediary,
            Err(error) => return Err(error),
        };
        // Identity trait implementation for our type, which provides error contexts with its name etc
        let struct_impl_identity_trait_code = implementation::identity::TraitImpl::gen(implementation::identity::trait_impl::ParameterStruct{
            definition_struct_ref: &struct_definition_ref,
            intermediary_struct_ref: &struct_intermediary,
        });

        // Intermediary class holding Option<Data> fields during deserialization
        let struct_data_context_intermediary = match intermediary::DataContext::try_from(struct_definition_ref) {
            Ok(struct_data_context_intermediary) => struct_data_context_intermediary,
            Err(error) => return Err(error),
        };
        // Struct definition to implement flexpiler::Deserialization on
        let struct_deserializer_intermediary = intermediary::deserializer::Struct::from(struct_definition_ref);

        let data_context_code = implementation::DataContext::gen(&struct_data_context_intermediary);
        let data_context_impl_default_code = implementation::data_context::DefaultImpl::gen(&struct_data_context_intermediary);
        let data_context_impl_tryinto_result_code = implementation::data_context::TryIntoResultImpl::gen(implementation::data_context::try_into_result_impl::ParameterStruct{
            struct_definition_ref,
            struct_intermediary_ref: &struct_intermediary,
            struct_deserializer_intermediary_ref: &struct_deserializer_intermediary,
            data_context_intermediary_ref: &struct_data_context_intermediary,
        });

        let deserializer_code = implementation::Deserializer::gen(&struct_deserializer_intermediary);
        let deserializer_impl_deserializer_trait_code = implementation::deserializer::DeserializerTraitImpl::gen(implementation::deserializer::deserializer_trait_impl::ParameterStruct {
            struct_definition_ref,
            struct_intermediary_ref: &struct_intermediary,
            struct_data_context_intermediary_ref: &struct_data_context_intermediary,
            struct_deserializer_intermediary_ref: &struct_deserializer_intermediary,
        });

        let struct_impl_deserialization = implementation::struct_rustc::DeserializationImpl::gen(implementation::struct_rustc::deserialization_impl::ParameterStruct{
            struct_definition_ref,
            struct_deserializer_intermediary_ref: &struct_deserializer_intermediary,
        });

        Ok(quote!{
            #struct_impl_identity_trait_code

            #data_context_code

            #deserializer_code

            #data_context_impl_default_code

            #data_context_impl_tryinto_result_code

            #deserializer_impl_deserializer_trait_code

            #struct_impl_deserialization
        })
    }
}


impl<'a> implementation::TryTrait<&'a definition::Enum> for Flexpiler {
    type Error = error::Error<'a>;

    fn try_gen(enum_definition_ref: &'a definition::Enum) -> std::result::Result<proc_macro2::TokenStream, Self::Error> {
        use std::convert::TryFrom;
        use implementation::Trait;

        let enum_intermediary = match intermediary::Enum::try_from(enum_definition_ref) {
            Ok(enum_intermediary) => enum_intermediary,
            Err(error) => return Err(error),
        };
        // Identity trait implementation for our type, which provides error contexts with its name etc
        let enum_impl_identity_trait_code = implementation::identity::TraitImpl::gen(implementation::identity::trait_impl::ParameterEnum{
            definition_enum_ref: enum_definition_ref,
            intermediary_enum_ref: &enum_intermediary,
        });
        let enum_variant_complex_data_context_vec = {
            let mut vec = Vec::new();
            //let variant_complex_ref_iter: std::slice::Iter<'a, > = enum_intermediary.variant_complex_vec.iter();

            for variant_complex_ref in enum_intermediary.variant_complex_vec.iter() {
                let data_context = match intermediary::DataContext::try_from(intermediary::data_context::ParameterEnumVariantComplex{
                    enum_definition_ref,
                    variant_complex_enum_intermediary_ref: variant_complex_ref,
                }) {
                    Ok(data_context) => data_context,
                    Err(error) => return Err(error),
                };
                vec.push(data_context);
            }
            vec
        };
        let enum_deserializer_intermediary = intermediary::deserializer::Enum::from(enum_definition_ref);

        let deserializer_code = implementation::deserializer::Deserializer::gen(&enum_deserializer_intermediary);
        let enum_data_context_code_vec = {
            let mut vec = Vec::new();
            for data_context_ref in enum_variant_complex_data_context_vec.iter() {
                vec.push(implementation::DataContext::gen(data_context_ref));
            }
            vec
        };
        let enum_data_context_default_code_vec = {
            let mut vec = Vec::new();
            for data_context_ref in enum_variant_complex_data_context_vec.iter() {
                vec.push(implementation::data_context::DefaultImpl::gen(data_context_ref));
            }
            vec
        };
        let enum_data_context_impl_tryinto_result_code_vec = {
            let mut vec = Vec::new();
            for (enum_variant_complex_intermediary_ref, data_context_intermediary_ref) in enum_intermediary.variant_complex_vec.iter()
                .zip(enum_variant_complex_data_context_vec.iter()) {
                vec.push(implementation::data_context::TryIntoResultImpl::gen(implementation::data_context::try_into_result_impl::ParameterEnum{
                    enum_definition_ref,
                    enum_deserializer_intermediary_ref: &enum_deserializer_intermediary,
                    enum_variant_complex_intermediary_ref,
                    data_context_intermediary_ref,
                }));
            }
            vec
        };
        let deserializer_impl_deserializer_trait_code = implementation::deserializer::DeserializerTraitImpl::gen(implementation::deserializer::deserializer_trait_impl::ParameterEnum{
            enum_definition_ref,
            enum_intermediary_ref: &enum_intermediary,
            enum_deserializer_intermediary_ref: &enum_deserializer_intermediary,
            enum_variable_complex_data_context_intermediary_vec_ref: &enum_variant_complex_data_context_vec,
        });

        let enum_impl_deserialization_code = implementation::struct_rustc::DeserializationImpl::gen(implementation::struct_rustc::deserialization_impl::ParameterEnum{
            enum_definition_ref,
            enum_deserializer_intermediary_ref: &enum_deserializer_intermediary,
        });

        Ok(quote!{
            #enum_impl_identity_trait_code

            #deserializer_code

            #(#enum_data_context_code_vec)*

            #(#enum_data_context_default_code_vec)*

            #(#enum_data_context_impl_tryinto_result_code_vec)*

            #deserializer_impl_deserializer_trait_code

            #enum_impl_deserialization_code
        })
    }
}
