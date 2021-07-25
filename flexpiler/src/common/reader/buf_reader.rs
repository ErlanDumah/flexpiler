// std
use std::fs::File;

// use internal
use crate::reader;


pub struct BufReader {
    buf_pointer: usize,
    buf_len: usize,
    buffer: [u8; 256],

    buf_reader: std::io::BufReader<File>,
}


impl BufReader {
    pub fn from_buf_reader(buf_reader: std::io::BufReader<File>) -> BufReader
    {
        BufReader {
            buf_len: 0,
            buf_pointer: 0,
            buffer: [0; 256],

            buf_reader,
        }
    }
}


impl From<std::io::BufReader<File>> for BufReader {
    fn from(std_buf_reader: std::io::BufReader<File>) -> Self {
        return Self::from_buf_reader(std_buf_reader);
    }
}

impl reader::Trait for BufReader {
    fn read(&mut self) -> reader::Result {
        use std::io::Read;

        if self.buf_pointer >= self.buf_len {
            let result = self.buf_reader.read(&mut self.buffer);

            match result {
                Ok(len) => {
                    if len == 0 {
                        return reader::Result::EndOfFile;
                    }
                    self.buf_len = len;
                    self.buf_pointer = 0;
                },
                Err(error) => {
                    return reader::Result::Err(
                        reader::Error::StdIoError(error)
                    );
                }
            }
        }

        let byte = self.buffer[self.buf_pointer];
        self.buf_pointer += 1;

        print!("<{}, {}>", byte, byte as char);
        return reader::Result::Ok(byte);
    }
}
