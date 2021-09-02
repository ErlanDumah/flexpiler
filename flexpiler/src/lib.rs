//! # Flexpiler
//!
//! A library that aims to maximize customisation of serialization.
//!
//! Most notably this library provides the derive macro ```flexpiler::Deserialize``` as well as some common
//! implementations in the ```flexpiler::common``` namespace.
//!
//! Currently the project only supports deserialization of a format that mirrors how structs and enums
//! are initialized in the rust language.
//!
//! # examples
//!
//! ```
//! #[derive(flexpiler::Deserialize)]
//! struct Example {
//!   field_i32: i32,
//! }
//!
//! let mut string_reader = flexpiler::common::reader::String::from("Example{ field_i32: 5 }");
//!
//! let example = match Example::deserialize(&mut string_reader) {
//!     Ok(example) => example,
//!     Err(error) => {
//!         assert!(false, "deserialisation failed: {}", error);
//!     }
//! };
//!
//! assert_eq!(example.field_i32, 5);
//! ```
//!


pub mod block;
pub mod common;
pub mod deserializer;
pub use error::Error;
pub mod error;
pub mod format;
pub mod identity;
pub mod parser;
pub mod reader;


///
/// A trait that associates a type with its flexpiler Deseriaizer type
///
/// The flexpiler_derive macro will generate a custom Deserializer type and then
/// impl this trait for your type, associating it with the auto-generated deserializer type.
///
pub trait Deserialization<FormatType: crate::format::Trait>: Sized {
    type Deserializer: crate::deserializer::Trait<Self, FormatType::DeserializerContext, FormatType::ErrorSource>;
}


///
/// A trait that makes a type support the flexpiler deserialize(reader) function.
///
/// It is recommended to not manually impl this trait for a type and instead opt
/// to impl flexpiler::Deserialization and move the deserialization into a type that
/// has an impl for flexpiler::deserializer::Trait.
///
pub trait Deserialize<FormatType: crate::format::Trait>: Sized {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> Result<Self, error::Error<FormatType::ErrorSource>>
    where ReaderType: reader::Trait;
}


impl<DataType, FormatType> Deserialize<FormatType> for DataType
where DataType: Deserialization<FormatType>,
      FormatType: crate::format::Trait,
      DataType::Deserializer: crate::deserializer::identity::Trait<DataType, FormatType>
                              + crate::deserializer::context::Trait<DataType, FormatType>,
      FormatType::ErrorSource: crate::error::source::Trait
{
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType) -> Result<Self, error::Error<FormatType::ErrorSource>> where ReaderType: reader::Trait {
        use error::Trait as ErrorTrait;
        use error::propagation::Trait as PropagationTrait;
        use deserializer::Trait as DeserializerTrait;
        use deserializer::context::Trait as ContextTrait;
        use deserializer::identity::Trait as IdentityTrait;

        return match DataType::Deserializer::deserialize(reader_mut_ref) {
            crate::deserializer::Result::DataFound{ data, .. } => {
                Ok(data)
            },
            crate::deserializer::Result::NoDataFound { .. } => {
                let unexpected_no_content = crate::error::source::common::UnexpectedNoContent {
                    definition_expected: <DataType::Deserializer as crate::deserializer::identity::Trait<DataType, FormatType>>::data_definition(),
                };

                let common_error_source = <crate::error::source::common::UnexpectedNoContent as Into<crate::error::source::Common>>::into(unexpected_no_content);
                let error = Error::gen(common_error_source)
                    .propagate(<DataType::Deserializer as crate::deserializer::context::Trait<DataType, FormatType>>::context_general());

                Err(error)
            }
            crate::deserializer::Result::Err(error) => Err(error),
        }
    }
}


#[macro_use]
extern crate flexpiler_derive;
pub use flexpiler_derive::*;
use crate::error::Trait;


#[cfg(test)]
mod tests {
    macro_rules! matches(
        ($e:expr, $p:pat) => (
            match $e {
                $p => true,
                _ => false
            }
        )
    );

    #[test]
    fn common_reader_string_test() {
        use crate::common;
        use crate::reader;
        use crate::reader::Trait;

        let mut reader = common::reader::String::from("Blba");

        println!("'B' as u8: {}", 'B' as u8);
        println!("'l' as u8: {}", 'l' as u8);
        println!("'b' as u8: {}", 'b' as u8);
        println!("'a' as u8: {}", 'a' as u8);

        assert!(matches![reader.read(), reader::Result::Ok(66)]);
        assert!(matches![reader.read(), reader::Result::Ok(108)]);
        assert!(matches![reader.read(), reader::Result::Ok(98)]);
        assert!(matches![reader.read(), reader::Result::Ok(97)]);
        assert!(matches![reader.read(), reader::Result::EndOfFile]);
    }

    #[test]
    fn common_deserializer_string_test() {
        use crate::common;
        use crate::reader;
        use crate::reader::Trait;

        use crate::Deserialize;

        let mut reader = common::reader::String::from("\"toss a witch\"");
        let deserialized_result = std::string::String::deserialize(&mut reader);
        assert!(deserialized_result.is_ok());
        if let Ok(deserialized) = deserialized_result {
            println!("Deserialized string: {}", deserialized);
            assert_eq!(deserialized.as_str(), "toss a witch");
        }
    }
}
