use crate::{error, Error};


pub mod context;
pub mod identity;


///
/// The result of a flexpiler Deserializer type.
///
/// flexpiler's deserialization algorithm is based on the idea of a uni-directional iterator through
/// the data, and to keep the logic on-the-fly as minimal as possible. That means we can never "go
/// back" to data we just parsed.
///
/// The deserialization is usually broken up into blocks of data separated by a certain ContextType
/// and randomly being accompanied by any number of pretty formatting. An example of this would be:
/// ```'TestStruct' -> '\t' -> '{' -> '\n' -> 'field:' -> ' ' -> '"somestring"' -> ',' -> '\n' -> '}'```
/// The algorithm counts on being able to call other deserialization::Trait<...> implementations to
/// deserialize sub types, and thereby a ContextType is shipped with the return type to determine if
/// a crucial struct-ending context like '}' was parsed during sub-deserialization.
///
/// To accomodate types of variable element size like vec![(any number of elements,)*] we need the
/// return type NoDataFound that is sometimes recoverable.
///
pub enum Result<DataType, ContextType, ErrorType> {
    DataFound { data: DataType, context: ContextType },
    NoDataFound { context: ContextType },
    Err(ErrorType),
}


///
/// The trait to be implemented by a flexpiler Deserializer type.
///
/// A deserializer type also has to impl context::Trait to account for common errors constructed
/// from outside like a flexpiler::error::Common::UnexpectedNoContent
///
pub trait Trait<DataType, FormatType>
where FormatType: crate::format::Trait {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> Result<DataType, FormatType::DeserializerContext, crate::Error<FormatType::ErrorSource>>
    where ReaderType: crate::reader::Trait;
}

