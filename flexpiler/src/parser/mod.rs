pub mod error;
pub use error::Error;

use crate::block;
use crate::reader;
use crate::reader::Trait;


pub trait Parse {
    type Result;

    fn parse<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> std::result::Result<Self::Result, crate::parser::Error>
    where ReaderType: crate::reader::Trait;
}


pub fn parse<
    BlockType,
    ReaderType
> (reader_mut_ref: &mut ReaderType)
   -> Result<BlockType::Result, Error>
where BlockType: block::Trait + Default,
      ReaderType: reader::Trait
{
    let mut block = BlockType::default();

    loop {
        let read_byte = match reader_mut_ref.read() {
            reader::Result::Ok(byte) => byte,
            reader::Result::EndOfFile => {
                println!("Encountered end of file");
                break;
            }
            reader::Result::Err(error) => {
                println!("Encountered IO error");
                break;
            }
        };

        let advance_result = block.advance_result(read_byte);

        match advance_result {
            block::AdvanceResult::Continuous => {},
            block::AdvanceResult::Error => {
                break;
            },
            block::AdvanceResult::Finished => {
                break;
            }
        }
    }

    block.into_result()
}
