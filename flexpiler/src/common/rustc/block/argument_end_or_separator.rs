use super::constants::*;

use crate::block;
use crate::error::ExpectedEntries;
use crate::parser::error;
use crate::reader;


enum Context {
    Initial,
    Comment,
    FinishArgumentEnd,
    FinishSeparator,
    Error,
}

pub enum Result {
    ArgumentEnd,
    Separator,
}

pub struct ArgumentEndOrSeparator {
    context: Context,
    error_source: error::Source,
}


impl ArgumentEndOrSeparator {
    pub fn new() -> ArgumentEndOrSeparator {
        ArgumentEndOrSeparator {
            context: Context::Initial,
            error_source: error::Source::NoContent,
        }
    }
}


impl Default for ArgumentEndOrSeparator {
    fn default() -> Self {
        return ArgumentEndOrSeparator::new();
    }
}


impl block::Trait for ArgumentEndOrSeparator {
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

            // On encountering a declaration nominator as first character
            // Throw error for not allowed empty declaration
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::FinishArgumentEnd;
                return block::AdvanceResult::Finished;
            },

            (&Context::Initial,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::FinishSeparator;
                return block::AdvanceResult::Finished;
            },

            // On encountering the first non ignorable | comment char
            // Change deserializer to string and add read_byte as first character
            (&Context::Initial,
                _) => {
                self.error_source = error::Source::UnexpectedToken(
                    error::UnexpectedToken{
                        token_expected_entries: ExpectedEntries::from(vec![DENOMINATOR_DATA_START as char]),
                        token_found: read_byte as char,
                    }
                );
                self.context = Context::Error;
                return block::AdvanceResult::Error;
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

            (&Context::FinishSeparator, _)
            | (&Context::FinishArgumentEnd, _) => {
                return block::AdvanceResult::Finished;
            },

            (&Context::Error,
                _) => {
                return block::AdvanceResult::Error;
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Source> {
        match self.context {
            Context::Initial
            | Context::Comment
            | Context::Error => {
                return Err(self.error_source);
            },
            Context::FinishSeparator => {
                return Ok(Result::Separator);
            },
            Context::FinishArgumentEnd => {
                return Ok(Result::ArgumentEnd);
            },
        }
    }
}