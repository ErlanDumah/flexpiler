use crate::reader;


pub struct String<'a> {
    chars: std::str::Chars<'a>,
}


impl<'a> From<&'a std::string::String> for String<'a> {
    fn from(string_ref: &'a std::string::String) -> Self {
        String {
            chars: string_ref.chars()
        }
    }
}


impl<'a> From<&'a str> for String<'a> {
    fn from(str_ref: &'a str) -> Self {
        String {
            chars: str_ref.chars()
        }
    }
}


impl<'a> reader::Trait for String<'a> {
    fn read(&mut self) -> reader::Result {
        match self.chars.next() {
            Some('\u{0306}')
            | None => {
                return reader::Result::EndOfFile;
            }

            Some(character) => {
                return reader::Result::Ok(character as u8);
            }
        }
    }
}

