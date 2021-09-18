
pub trait Trait {
    fn line_count(&self) -> usize;
    fn char_count(&self) -> usize;
}


pub struct Transcribe<ReaderType> {
    reader_impl: ReaderType,

    line_count: usize,
    char_count: usize,
}


impl<ReaderType> From<ReaderType> for Transcribe<ReaderType> {
    fn from(reader_impl: ReaderType) -> Self {
        Transcribe {
            reader_impl,
            char_count: 0,
            line_count: 0,
        }
    }
}


impl<ReaderType> crate::reader::Trait for Transcribe<ReaderType>
where ReaderType: crate::reader::Trait {
    fn read(&mut self) -> crate::reader::Result {
        let result = self.reader_impl.read();
        match &result {
            &crate::reader::Result::Ok(crate::common::rustc::block::constants::DENOMINATOR_NEW_LINE) => {
                self.line_count = self.line_count + 1;
                self.char_count = 0;
            },
            _ => {
                self.char_count = self.char_count + 1;
            }
        }
        return result;
    }
}


impl<ReaderType> Trait for Transcribe<ReaderType> {
    fn line_count(&self) -> usize {
        return self.line_count;
    }

    fn char_count(&self) -> usize {
        return self.char_count;
    }
}
