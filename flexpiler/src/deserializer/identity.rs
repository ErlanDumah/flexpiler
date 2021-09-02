
///
/// A trait that allows a deserializer to identify its object type to be deserialized
///
/// Currently used for error context generation
///
pub trait Trait<DataType, FormatType> {
    fn data_definition() -> std::string::String;
}


impl<DataType, DeserializerType, FormatType> Trait<DataType, FormatType> for DeserializerType
where DataType: crate::identity::Trait,
      FormatType: crate::format::Trait,
      DeserializerType: crate::deserializer::Trait<DataType, FormatType::DeserializerContext, FormatType::ErrorSource> {
    fn data_definition() -> String {
        return <DataType as crate::identity::Trait>::definition();
    }
}
