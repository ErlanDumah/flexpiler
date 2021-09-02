use std::fmt::{Display, Formatter, Pointer};

pub struct UnexpectedNoContent {
    pub definition_expected: std::string::String,
}


pub enum Common {
    UnexpectedNoContent(UnexpectedNoContent),
}


impl std::fmt::Display for UnexpectedNoContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected content of type {} but none was found.",
                   self.definition_expected)
    }
}


impl std::fmt::Display for Common {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match &self {
            &Common::UnexpectedNoContent(ref unexpected_no_content_ref) => {
                unexpected_no_content_ref.fmt(f)
            }
        }
    }
}


impl Into<Common> for UnexpectedNoContent {
    fn into(self) -> Common {
        Common::UnexpectedNoContent(self)
    }
}
