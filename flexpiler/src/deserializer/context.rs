
use crate::error;
use crate::identity;
use crate::error::Context;


///
/// A trait that provides functions to determine the context of a deserializer in case of an error
///
pub trait Trait<DataType, FormatType> {
    /** General error context of this deserializer */
    fn context() -> error::Context;
}


pub trait FieldTrait<DataType, FormatType> {
    /** Context of an error that occurred within a field of the struct */
    fn context_field(field_name_ref: &str) -> crate::error::Context;
}


pub trait VariantTrait<DataType, FormatType> {
    /** Context of an error from an enum variant */
    fn context_variant(variant_name_ref: &str) -> crate::error::Context;
}


pub trait VariantFieldTrait<DataType, FormatType> {
    /** Context of an error from an enum variant */
    fn context_variant_field(variant_name_ref: &str, field_name_ref: &str) -> crate::error::Context;
}


impl<DataType, DeserializerType, FormatType> Trait<DataType, FormatType> for DeserializerType
where FormatType: crate::format::Trait,
      DeserializerType: crate::deserializer::identity::Trait<DataType, FormatType> {
    fn context() -> error::Context {
        error::Context {
            trace: <Self as crate::deserializer::identity::Trait<DataType, FormatType>>::data_definition(),
        }
    }
}


impl<DataType, DeserializerType, FormatType> FieldTrait<DataType, FormatType> for DeserializerType
where FormatType: crate::format::Trait,
      DeserializerType: crate::deserializer::identity::Trait<DataType, FormatType> {
    fn context_field(field_name_ref: &str) -> Context {
        error::Context {
            trace: std::string::String::from(format!("{}[{}]",
                DeserializerType::data_definition(),
                field_name_ref,
            ))
        }
    }
}


impl<DataType, DeserializerType, FormatType> VariantTrait<DataType, FormatType> for DeserializerType
    where FormatType: crate::format::Trait,
          DeserializerType: crate::deserializer::identity::Trait<DataType, FormatType> {
    fn context_variant(variant_name_ref: &str) -> Context {
        error::Context {
            trace: std::string::String::from(format!("{}::{}",
                DeserializerType::data_definition(),
                variant_name_ref,
            ))
        }
    }
}


impl<DataType, DeserializerType, FormatType> VariantFieldTrait<DataType, FormatType> for DeserializerType
    where FormatType: crate::format::Trait,
          DeserializerType: crate::deserializer::identity::Trait<DataType, FormatType> {
    fn context_variant_field(variant_name_ref: &str, field_name_ref: &str) -> Context {
        error::Context {
            trace: std::string::String::from(format!("{}::{}[{}]",
                DeserializerType::data_definition(),
                variant_name_ref,
                field_name_ref,
            ))
        }
    }
}
