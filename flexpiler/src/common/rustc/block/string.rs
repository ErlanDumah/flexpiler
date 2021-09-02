use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;
use crate::error::ExpectedEntries;


enum Context {
    Initial,

    Comment,

    StringDenominated,
    StringFreestanding,

    FinishFreestanding,
    FinishArgumentEnd,
    FinishDataEnd,
    FinishSeparator,

    Error,
}


pub enum Finish {
    Freestanding,
    ArgumentEnd,
    DataEnd,
    Separator,
}


pub struct Result {
    pub finish: Finish,
    pub string: std::string::String,
}


pub struct String {
    error_source: error::Source,
    context: Context,
    string: std::string::String,
}


impl Into<crate::common::rustc::deserializer::Context> for Finish {
    fn into(self) -> crate::common::rustc::deserializer::Context {
        return match self {
            Finish::DataEnd => crate::common::rustc::deserializer::Context::DataEnd,
            Finish::Freestanding => crate::common::rustc::deserializer::Context::Freestanding,
            Finish::ArgumentEnd => crate::common::rustc::deserializer::Context::ArgumentEnd,
            Finish::Separator => crate::common::rustc::deserializer::Context::Separator,
        }
    }
}


impl String {
    pub fn new() -> String {
        String {
            error_source: error::Source::NoContent,
            context: Context::Initial,
            string: std::string::String::new(),
        }
    }
}


impl Default for String {
    fn default() -> Self {
        return String::new();
    }
}


impl block::Trait for String {
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
                DENOMINATOR_STRING) => {
                self.context = Context::StringDenominated;
                return block::AdvanceResult::Continuous;
            },

            // On encountering regular
            (&Context::Initial,
                _) => {
                self.context = Context::StringFreestanding;
                self.string.push(read_byte as char);
                return block::AdvanceResult::Continuous;
            },
            
            (&Context::Initial,
                _) => {
                self.error_source = error::Source::UnexpectedToken(error::UnexpectedToken{
                    token_expected_entries: ExpectedEntries::from(vec![DENOMINATOR_STRING as char]),
                    token_found: read_byte as char,
                });
                return block::AdvanceResult::Error;
            },

            (&Context::StringDenominated,
                DENOMINATOR_STRING) => {
                self.context = Context::FinishFreestanding;
                return block::AdvanceResult::Finished;
            },

            // On encountering regular
            (&Context::StringDenominated,
                _) => {
                self.string.push(read_byte as char);
                return block::AdvanceResult::Continuous;
            },

            (&Context::StringFreestanding,
                DENOMINATOR_NEW_LINE)
            | (&Context::StringFreestanding,
                DENOMINATOR_SPACE)
            | (&Context::StringFreestanding,
                DENOMINATOR_TAB) => {
                self.context = Context::FinishFreestanding;
                return block::AdvanceResult::Finished;
            },

            (&Context::StringFreestanding,
                DENOMINATOR_DATA_END) => {
                self.context = Context::FinishDataEnd;
                return block::AdvanceResult::Finished;
            },

            (&Context::StringFreestanding,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::FinishArgumentEnd;
                return block::AdvanceResult::Finished;
            },

            (&Context::StringFreestanding,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::FinishSeparator;
                return block::AdvanceResult::Finished;
            },

            (&Context::StringFreestanding,
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
            | (&Context::FinishArgumentEnd, _)
            | (&Context::FinishDataEnd, _)
            | (&Context::FinishFreestanding, _) => {
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
            | Context::StringFreestanding
            | Context::StringDenominated
            | Context::Error => {
                return Err(self.error_source);
            },
            Context::FinishFreestanding => {
                return Ok(Result {
                    finish: Finish::Freestanding,
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
            Context::FinishArgumentEnd => {
                return Ok(Result {
                    finish: Finish::ArgumentEnd,
                    string: self.string,
                });
            },
        }
    }
}