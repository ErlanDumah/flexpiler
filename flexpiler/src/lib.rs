pub mod block;
pub mod common;
pub mod deserializer;
pub use error::Error;
pub mod error;
pub mod format;
pub mod parser;
pub mod reader;


pub trait Deserialization<FormatType: crate::format::Trait>: Sized {
    type Deserializer: crate::deserializer::Trait<Self, FormatType::DeserializerContext>;
}


pub trait Deserialize<FormatType>: Sized {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> Result<Self, error::Error>
    where ReaderType: reader::Trait;
}


impl<DataType, FormatType> Deserialize<FormatType> for DataType
where DataType: Deserialization<FormatType>,
      FormatType: crate::format::Trait,
{
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType) -> Result<Self, Error> where ReaderType: reader::Trait {
        use deserializer::Trait;

        return match DataType::Deserializer::deserialize(reader_mut_ref) {
            Ok(deserializer_result) => {
                Ok(deserializer_result.data)
            },
            Err(error) => Err(error),
        }
    }
}


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