use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;


enum Context {
    Initial,
    InitialComment,

    IdentifierString,

    FinishedWithFreestanding,
    FinishedWithArgumentStart,
    FinishedWithDataStart,
    FinishedWithSeparator,

    Error,
}


#[derive(Clone)]
pub enum Finish {
    // Block may be successfully parsed even if the end of data is reached
    NoContext,

    Freestanding,
    Separator,
    ArgumentData,
    ComplexData,
}


#[derive(Clone)]
pub struct Result {
    pub identifier_string: std::string::String,
    pub finish: Finish,
}


pub struct IdentifierWithVariableFinish {
    error_source: error::Source,
    context: Context,
    string: std::string::String,
}


impl Into<crate::common::rustc::deserializer::Context> for Finish {
    fn into(self) -> crate::common::rustc::deserializer::Context {
        return match self {
            Finish::NoContext => crate::common::rustc::deserializer::Context::Freestanding,
            Finish::Freestanding => crate::common::rustc::deserializer::Context::Freestanding,
            Finish::Separator => crate::common::rustc::deserializer::Context::Separator,
            Finish::ArgumentData => crate::common::rustc::deserializer::Context::ArgumentStart,
            Finish::ComplexData => crate::common::rustc::deserializer::Context::DataStart,
        }
    }
}


impl std::fmt::Display for Finish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Finish::NoContext => {
                write!(f, "NoContext")
            },
            &Finish::Freestanding => {
                write!(f, "Freestanding")
            },
            &Finish::Separator => {
                write!(f, "NoData")
            },
            &Finish::ArgumentData => {
                write!(f, "ArgumentData")
            },
            &Finish::ComplexData => {
                write!(f, "ComplexData")
            },
        }
    }
}


impl IdentifierWithVariableFinish {
    pub fn new() -> IdentifierWithVariableFinish {
        IdentifierWithVariableFinish {
            error_source: error::Source::NoContent,
            context: Context::Initial,
            string: std::string::String::new(),
        }
    }
}


impl Default for IdentifierWithVariableFinish {
    fn default() -> Self {
        return Self::new();
    }
}


impl block::Trait for IdentifierWithVariableFinish {
    type Result = Result;

    fn advance_result(&mut self, read_byte: u8) -> block::AdvanceResult {
        return match (&self.context, read_byte) {
            // Accept comments in initial state
            (&Context::Initial,
                DENOMINATOR_COMMENT) => {
                self.context = Context::InitialComment;
                block::AdvanceResult::Continuous
            },

            // Ignore newline, space, tab at initial state
            (&Context::Initial,
                DENOMINATOR_NEW_LINE)
            | (&Context::Initial,
                DENOMINATOR_SPACE)
            | (&Context::Initial,
                DENOMINATOR_TAB) => {
                block::AdvanceResult::Continuous
            },

            // On encountering the first non ignorable | comment char
            // Change deserializer to string and add read_byte as first character
            (&Context::Initial,
                _) => {
                self.string.push(read_byte as char);
                self.context = Context::IdentifierString;
                block::AdvanceResult::Continuous
            },

            (&Context::IdentifierString,
                DENOMINATOR_SPACE)
            | (&Context::IdentifierString,
                DENOMINATOR_NEW_LINE)
            | (&Context::IdentifierString,
                DENOMINATOR_TAB) => {
                self.context = Context::FinishedWithFreestanding;
                block::AdvanceResult::Finished
            },

            (&Context::IdentifierString,
                DENOMINATOR_DATA_START) => {
                self.context = Context::FinishedWithDataStart;
                block::AdvanceResult::Finished
            },

            (&Context::IdentifierString,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedWithArgumentStart;
                block::AdvanceResult::Finished
            },

            (&Context::IdentifierString,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::FinishedWithSeparator;
                block::AdvanceResult::Finished
            },

            // On encountering regular
            (&Context::IdentifierString,
                _) => {
                self.string.push(read_byte as char);
                block::AdvanceResult::Continuous
            },

            // On newline from comment reset deserializer
            (&Context::InitialComment,
                DENOMINATOR_NEW_LINE) => {
                self.context = Context::Initial;
                block::AdvanceResult::Continuous
            },

            // Ignore any other character in a comment
            (&Context::InitialComment,
                _) => {
                block::AdvanceResult::Continuous
            },

            (&Context::FinishedWithFreestanding,
                _)
            | (&Context::FinishedWithDataStart,
                _)
            | (&Context::FinishedWithArgumentStart,
                _)
            | (&Context::FinishedWithSeparator,
                _) => {
                block::AdvanceResult::Finished
            },

            (&Context::Error,
                _) => {
                block::AdvanceResult::Error
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Source> {
        let data_type = match self.context {
            Context::Initial
            | Context::InitialComment
            | Context::Error => {
                return Err(self.error_source);
            },
            Context::IdentifierString => {
                Finish::NoContext
            },
            Context::FinishedWithFreestanding => {
                Finish::Freestanding
            },
            Context::FinishedWithDataStart => {
                Finish::ComplexData
            },
            Context::FinishedWithArgumentStart => {
                Finish::ArgumentData
            },
            Context::FinishedWithSeparator => {
                Finish::Separator
            },
        };

        Ok(Result {
            identifier_string: self.string,
            finish: data_type,
        })
    }
}
