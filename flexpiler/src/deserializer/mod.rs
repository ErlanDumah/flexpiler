use crate::{error, Error};


pub struct Result<DataType, ContextType> {
    pub data: DataType,
    pub context: ContextType,
}


pub trait Trait<DataType, ContextType> {
    fn deserialize<ReaderType>(reader_mut_ref: &mut ReaderType)
        -> std::result::Result<Result<DataType, ContextType>, crate::Error>
    where ReaderType: crate::reader::Trait;
}

