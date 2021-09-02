
use crate::error;
use crate::identity;
use crate::error::Context;


///
/// A trait that provides functions to determine the context of a deserializer in case of an error
///
pub trait Trait<DataType, FormatType> {
    /** General error context of this deserializer */
    fn context_general() -> error::Context;
}


///
///
///
pub trait EnumTrait<DataType, FormatType>: Trait<DataType, FormatType> {
    /** Context of an error that occurred in relation to a specific enum variant entry */
    fn context_entry_general(enum_entry_name: &str) -> crate::error::Context;

    /** Context of an error that occurred within a field of a complex enum variant entry */
    fn context_entry_field(enum_entry_name: &str,
                           field_name: &str) -> crate::error::Context;
}


///
///
///
pub trait StructTrait<DataType, FormatType>: Trait<DataType, FormatType> {
    /** Context of an error that occurred within a field of the struct */
    fn context_field(field_name: &str) -> crate::error::Context;
}


impl<DataType, DeserializerType, FormatType> Trait<DataType, FormatType> for DeserializerType
where FormatType: crate::format::Trait,
      DeserializerType: crate::deserializer::identity::Trait<DataType, FormatType> {
    fn context_general() -> error::Context {
        error::Context {
            trace: <Self as crate::deserializer::identity::Trait<DataType, FormatType>>::data_definition(),
        }
    }
}
