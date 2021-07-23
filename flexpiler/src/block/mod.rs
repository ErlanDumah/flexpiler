
use crate::parser::{error, Parse};
use std::io::Read;


pub enum AdvanceResult {
    // There are more bytes to be parsed
    Continuous,
    // There was an error
    Error,
    // The block is finished
    Finished,
}


//TODO: make block::Trait<ResultType> be a supertype of Into<std::Result<ResultType, ErrorType>>
pub trait Trait: Default {
    type Result;

    fn advance_result(&mut self, read_byte: u8) -> AdvanceResult;
    fn into_result(self) -> Result<Self::Result, error::Error>;
}


impl<BlockType> crate::parser::Parse for BlockType
where BlockType: Trait + Default {
    type Result = <Self as Trait>::Result;

    fn parse<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> std::result::Result<Self::Result, crate::parser::Error>
    where ReaderType: crate::reader::Trait {
        crate::parser::parse::<
            Self,
            ReaderType
        >(reader_mut_ref)
    }
}
