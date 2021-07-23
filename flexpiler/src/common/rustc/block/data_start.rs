use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;
use crate::error::ExpectedEntries;


enum DataStartContext {
    Initial,
    Comment,
    Finished,
    Error,
}

pub struct Result {
    // empty
}

pub struct DataStart {
    context: DataStartContext,
    error: error::Error,
}


impl DataStart {
    pub fn new() -> DataStart {
        DataStart {
            context: DataStartContext::Initial,
            error: error::Error {
                error_source: error::Source::NoContent,
            },
        }
    }
}


impl Default for DataStart {
    fn default() -> Self {
        return DataStart::new();
    }
}


impl block::Trait for DataStart {
    type Result = Result;

    fn advance_result(&mut self, read_byte: u8) -> block::AdvanceResult {
        match (&self.context, read_byte) {
            // Accept comments in initial state
            (&DataStartContext::Initial,
                DENOMINATOR_COMMENT) => {
                self.context = DataStartContext::Comment;
                return block::AdvanceResult::Continuous;
            },

            // Ignore newline, space, tab at initial state
            (&DataStartContext::Initial,
                DENOMINATOR_NEW_LINE)
            | (&DataStartContext::Initial,
                DENOMINATOR_SPACE)
            | (&DataStartContext::Initial,
                DENOMINATOR_TAB) => {
                return block::AdvanceResult::Continuous;
            },

            // On encountering a declaration nominator as first character
            // Throw error for not allowed empty declaration
            (&DataStartContext::Initial,
                DENOMINATOR_DATA_START) => {
                self.context = DataStartContext::Finished;
                return block::AdvanceResult::Finished;
            },

            // On encountering the first non ignorable | comment char
            // Change deserializer to string and add read_byte as first character
            (&DataStartContext::Initial,
                _) => {
                self.error = error::Error {
                    error_source: error::Source::UnexpectedToken(
                        error::UnexpectedToken{
                            token_expected_entries: ExpectedEntries::from(vec![DENOMINATOR_DATA_START as char]),
                            token_found: read_byte as char,
                        }
                    ),
                };
                self.context = DataStartContext::Error;
                return block::AdvanceResult::Error;
            },

            // On newline from comment reset deserializer
            (&DataStartContext::Comment,
                DENOMINATOR_NEW_LINE) => {
                self.context = DataStartContext::Initial;
                return block::AdvanceResult::Continuous;
            },

            // Ignore any other character in a comment
            (&DataStartContext::Comment,
                _) => {
                return block::AdvanceResult::Continuous;
            },

            (&DataStartContext::Finished,
                _) => {
                return block::AdvanceResult::Finished;
            },

            (&DataStartContext::Error,
                _) => {
                return block::AdvanceResult::Error;
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Error> {
        match self.context {
            DataStartContext::Initial
            | DataStartContext::Comment
            | DataStartContext::Error => {
                return Err(self.error);
            }
            DataStartContext::Finished => {
                return Ok(Result {});
            },
        }
    }
}