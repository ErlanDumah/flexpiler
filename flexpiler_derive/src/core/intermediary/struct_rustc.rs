use crate::core::definition;
use crate::core::error;


pub struct Struct<'a> {
    pub ident_string: std::string::String,
    pub field_ident_vec: Vec<&'a syn::Ident>,
    pub field_string_vec: Vec<std::string::String>,
    pub field_type_vec: Vec<&'a syn::Type>,
}


impl<'a> std::convert::TryFrom<&'a definition::Struct> for Struct<'a> {
    type Error = error::Error<'a>;

    fn try_from(struct_definition_ref: &'a definition::Struct) -> std::result::Result<Self, Self::Error> {
        let mut field_ident_vec = Vec::new();
        let mut field_string_vec = Vec::new();
        let mut field_type_vec = Vec::new();
        for field_ref in struct_definition_ref.data_struct.fields.iter() {
            let field_ident_ref = match &field_ref.ident {
                &Some(ref value_ref) => value_ref,
                &None => {
                    return Err(
                        Self::Error::StructFieldUnnamed{
                            field_ref,
                        },
                    )
                }
            };

            field_ident_vec.push(field_ident_ref);
            field_string_vec.push(format!("{}", field_ident_ref));
            field_type_vec.push(&field_ref.ty );
        }

        Ok(Struct {
            ident_string: struct_definition_ref.ident.to_string(),
            field_ident_vec,
            field_string_vec,
            field_type_vec,
        })
    }
}
