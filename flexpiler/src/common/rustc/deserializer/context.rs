

#[derive(PartialEq)]
pub enum Context {
    Freestanding,   // Block was ended by a ' ', '\t' or '\n'
    Separator,      // Block ended with a ','
    ArgumentStart,  // Block ended with a '('
    ArgumentEnd,    // Block ended with a ')'
    DataStart,      // Block ended with a '{'
    DataEnd,        // Block ended with a '}'
}


impl std::fmt::Display for Context {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Context::Freestanding => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_SPACE)
            },
            Context::Separator => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_SEPARATOR)
            },
            Context::ArgumentStart => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_ARGUMENT_START)
            },
            Context::ArgumentEnd => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_ARGUMENT_END)
            },
            Context::DataStart => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_DATA_START)
            },
            Context::DataEnd => {
                write!(f, "'{}'", crate::common::rustc::block::constants::DENOMINATOR_DATA_END)
            },
        }
    }
}
