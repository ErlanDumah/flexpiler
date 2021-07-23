use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;


enum Context {
    Initial,

    Comment,
    String,
    FinishedFreestanding,
    FinishedWithDataStart,
    Error,
}


#[derive(Clone)]
pub enum Result {
    Freestanding(String),
    DataStartFinish(String),
}


pub struct IdentifierWithDataStartFinish {
    error: error::Error,
    context: Context,
    string: std::string::String,
}


impl IdentifierWithDataStartFinish {
    pub fn new() -> IdentifierWithDataStartFinish {
        IdentifierWithDataStartFinish {
            error: error::Error {
                error_source: error::Source::NoContent,
            },
            context: Context::Initial,
            string: std::string::String::new(),
        }
    }
}


impl Default for IdentifierWithDataStartFinish {
    fn default() -> Self {
        return Self::new();
    }
}


impl block::Trait for IdentifierWithDataStartFinish {
    type Result = Result;

    fn advance_result(&mut self, read_byte: u8) -> block::AdvanceResult {
        match (&self.context, read_byte) {
            // Accept comments in initial state
            (&Context::Initial,
                DENOMINATOR_COMMENT) => {
                self.context = Context::Comment;
                return block::AdvanceResult::Continuous;
            },

            // Ignore newline, space, tab at initial state
            (&Context::Initial,
                DENOMINATOR_NEW_LINE)
            | (&Context::Initial,
                DENOMINATOR_SPACE)
            | (&Context::Initial,
                DENOMINATOR_TAB) => {
                return block::AdvanceResult::Continuous;
            },

            // On encountering the first non ignorable | comment char
            // Change deserializer to string and add read_byte as first character
            (&Context::Initial,
                _) => {
                self.string.push(read_byte as char);
                self.context = Context::String;
                return block::AdvanceResult::Continuous;
            },

            (&Context::String,
                DENOMINATOR_SPACE)
            | (&Context::String,
                DENOMINATOR_NEW_LINE)
            | (&Context::String,
                DENOMINATOR_TAB) => {
                self.context = Context::FinishedFreestanding;
                return block::AdvanceResult::Finished;
            },

            (&Context::String,
                DENOMINATOR_DATA_START) => {
                self.context = Context::FinishedWithDataStart;
                return block::AdvanceResult::Finished;
            },

            // On encountering regular
            (&Context::String,
                _) => {
                self.string.push(read_byte as char);
                return block::AdvanceResult::Continuous;
            },

            // On newline from comment reset deserializer
            (&Context::Comment,
                DENOMINATOR_NEW_LINE) => {
                self.context = Context::Initial;
                return block::AdvanceResult::Continuous;
            },

            // Ignore any other character in a comment
            (&Context::Comment,
                _) => {
                return block::AdvanceResult::Continuous;
            },

            (&Context::FinishedWithDataStart,
                _) => {
                return block::AdvanceResult::Finished;
            },

            (&Context::FinishedFreestanding,
                _) => {
                return block::AdvanceResult::Finished;
            },

            (&Context::Error,
                _) => {
                return block::AdvanceResult::Error;
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Error> {
        match self.context {
            Context::Initial
            | Context::Comment
            | Context::String
            | Context::Error => {
                return Err(self.error);
            },
            Context::FinishedFreestanding => {
                return Ok(Result::Freestanding(self.string))
            },
            Context::FinishedWithDataStart => {
                return Ok(Result::DataStartFinish(self.string))
            },
        }
    }
}
