use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;
use crate::common::rustc::block::number::Context::FinishFreestanding;


enum Context {
    Initial,

    Comment,

    Number,

    FinishFreestanding,
    FinishArgumentEnd,
    FinishDataEnd,
    FinishSeparator,

    Error,
}


pub enum Finish {
    NoContext,

    Freestanding,
    ArgumentEnd,
    DataEnd,
    Separator,
}


pub struct Result {
    pub finish: Finish,
    pub string: std::string::String,
}


pub struct Number {
    error: error::Error,
    context: Context,
    string: std::string::String,
}


impl Into<crate::common::rustc::deserializer::Context> for Finish {
    fn into(self) -> crate::common::rustc::deserializer::Context {
        return match self {
            Finish::NoContext => crate::common::rustc::deserializer::Context::Freestanding,
            Finish::Freestanding => crate::common::rustc::deserializer::Context::Freestanding,
            Finish::ArgumentEnd => crate::common::rustc::deserializer::Context::ArgumentEnd,
            Finish::DataEnd => crate::common::rustc::deserializer::Context::DataEnd,
            Finish::Separator => crate::common::rustc::deserializer::Context::Separator,
        }
    }
}


impl Number {
    pub fn new() -> Number {
        Number {
            error: error::Error {
                error_source: error::Source::NoContent,
            },
            context: Context::Initial,
            string: std::string::String::new(),
        }
    }
}


impl Default for Number {
    fn default() -> Self {
        return Number::new();
    }
}


impl block::Trait for Number {
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

            // On encountering regular
            (&Context::Initial,
                _) => {
                self.context = Context::Number;
                self.string.push(read_byte as char);
                return block::AdvanceResult::Continuous;
            },

            (&Context::Number,
                DENOMINATOR_NEW_LINE)
            | (&Context::Number,
                DENOMINATOR_SPACE)
            | (&Context::Number,
                DENOMINATOR_TAB) => {
                self.context = Context::FinishFreestanding;
                return block::AdvanceResult::Finished;
            },

            (&Context::Number,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::FinishSeparator;
                return block::AdvanceResult::Finished;
            },

            (&Context::Number,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::FinishArgumentEnd;
                return block::AdvanceResult::Finished;
            },

            (&Context::Number,
                DENOMINATOR_DATA_END) => {
                self.context = Context::FinishDataEnd;
                return block::AdvanceResult::Finished;
            },

            (&Context::Number,
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

            (&Context::FinishSeparator, _)
            | (&Context::FinishDataEnd, _)
            | (&Context::FinishArgumentEnd, _)
            | (&Context::FinishFreestanding, _) => {
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
            | Context::Error => {
                return Err(self.error);
            },
            Context::Number => {
                return Ok(Result {
                    finish: Finish::NoContext,
                    string: self.string,
                })
            },
            Context::FinishFreestanding => {
                return Ok(Result {
                    finish: Finish::Freestanding,
                    string: self.string,
                });
            },
            Context::FinishArgumentEnd => {
                return Ok(Result {
                    finish: Finish::ArgumentEnd,
                    string: self.string,
                });
            },
            Context::FinishDataEnd => {
                return Ok(Result {
                    finish: Finish::DataEnd,
                    string: self.string,
                });
            },
            Context::FinishSeparator => {
                return Ok(Result {
                    finish: Finish::Separator,
                    string: self.string,
                });
            },
        }
    }
}