use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;
use crate::error::ExpectedEntries;


enum Context {
    Initial,
    Comment,

    FinishedSeparator,
    FinishedArgumentStart,
    FinishedArgumentEnd,
    FinishedDataStart,
    FinishedDataEnd,
    FinishedListStart,
    FinishedListEnd,

    Error,
}


pub enum Finish {
    Separator,
    ArgumentStart,
    ArgumentEnd,
    DataStart,
    DataEnd,
    ListStart,
    ListEnd,
}


pub struct Result {
    pub finish: Finish,
}

pub struct ContextDenominator {
    context: Context,
    error_source: error::Source,
}


impl ContextDenominator {
    pub fn new() -> ContextDenominator {
        ContextDenominator {
            context: Context::Initial,
            error_source: error::Source::NoContent,
        }
    }
}


impl Into<crate::common::rustc::deserializer::Context> for Finish {
    fn into(self) -> crate::common::rustc::deserializer::Context {
        return match self {
            Finish::ArgumentEnd => crate::common::rustc::deserializer::Context::ArgumentEnd,
            Finish::ArgumentStart => crate::common::rustc::deserializer::Context::ArgumentStart,
            Finish::DataEnd => crate::common::rustc::deserializer::Context::DataEnd,
            Finish::DataStart => crate::common::rustc::deserializer::Context::DataStart,
            Finish::ListEnd => crate::common::rustc::deserializer::Context::ListEnd,
            Finish::ListStart => crate::common::rustc::deserializer::Context::ListStart,
            Finish::Separator => crate::common::rustc::deserializer::Context::Separator,
        }
    }
}


impl Default for ContextDenominator {
    fn default() -> Self {
        return ContextDenominator::new();
    }
}


impl block::Trait for ContextDenominator {
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
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedArgumentStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::FinishedArgumentEnd;
                return block::AdvanceResult::Finished;
            },
            (&Context::Initial,
                DENOMINATOR_DATA_START) => {
                self.context = Context::FinishedDataStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::Initial,
                DENOMINATOR_DATA_END) => {
                self.context = Context::FinishedDataEnd;
                return block::AdvanceResult::Finished;
            },
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedArgumentStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedArgumentStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedArgumentStart;
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

            (&Context::FinishedArgumentEnd,
                _)
            | (&Context::FinishedArgumentStart,
                _)
            | (&Context::FinishedDataEnd,
                _)
            | (&Context::FinishedDataStart,
                _)
            | (&Context::FinishedListEnd,
                _)
            | (&Context::FinishedListStart,
                _)
            | (&Context::FinishedSeparator,
                _) => {
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
            }
            Context::FinishedArgumentEnd => {
                return Ok(Result{
                    finish: Finish::ArgumentEnd,
                });
            },
            Context::FinishedArgumentStart => {
                return Ok(Result{
                    finish: Finish::ArgumentStart,
                });
            },
            Context::FinishedDataEnd => {
                return Ok(Result{
                    finish: Finish::DataEnd,
                });
            },
            Context::FinishedDataStart => {
                return Ok(Result{
                    finish: Finish::DataStart,
                });
            },
            Context::FinishedListEnd => {
                return Ok(Result{
                    finish: Finish::ListEnd,
                });
            },
            Context::FinishedListStart => {
                return Ok(Result{
                    finish: Finish::ListStart,
                });
            },
            Context::FinishedSeparator => {
                return Ok(Result{
                    finish: Finish::Separator,
                });
            },
        }
    }
}