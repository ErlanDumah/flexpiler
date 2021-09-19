

#[derive(PartialEq)]
pub enum Context {
    Freestanding,   // Block was ended by a ' ', '\t', '\n' or the end of the reader
    Separator,      // Block ended with a ','
    ArgumentStart,  // Block ended with a '('
    ArgumentEnd,    // Block ended with a ')'
    DataStart,      // Block ended with a '{'
    DataEnd,        // Block ended with a '}'
    ListStart,      // Block ended with a '['
    ListEnd,        // Block ended with a ']'
}


impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Context::Freestanding => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_SPACE as char)
            },
            Context::Separator => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_SEPARATOR as char)
            },
            Context::ArgumentStart => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_ARGUMENT_START as char)
            },
            Context::ArgumentEnd => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_ARGUMENT_END as char)
            },
            Context::DataStart => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_DATA_START as char)
            },
            Context::DataEnd => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_DATA_END as char)
            },
            Context::ListStart => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_LIST_START as char)
            },
            Context::ListEnd => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_LIST_END as char)
            },
        }
    }
}
