
pub enum Error {
    StdIoError(std::io::Error)
}


pub enum Result {
    Ok(u8),
    EndOfFile,
    Err(Error)
}

pub trait Trait {
    fn read(&mut self) -> Result;
}
