use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;
use crate::error::ExpectedEntries;


enum Context {
    Initial,
    InitialComment,

    InitialArgumentEnd,
    InitialArgumentStart,
    InitialDataEnd,
    InitialDataStart,
    InitialListEnd,
    InitialListStart,
    InitialSeparator,

    StringDenominated,
    StringFreestanding,

    FinishedArgumentEnd,
    FinishedArgumentStart,
    FinishedDataEnd,
    FinishedDataStart,
    FinishedFreestanding,
    FinishedListEnd,
    FinishedListStart,
    FinishedSeparator,

    Error,
}


pub enum Finish {
    NoContext,

    ArgumentEnd,
    ArgumentStart,
    DataEnd,
    DataStart,
    Freestanding,
    ListEnd,
    ListStart,
    Separator,
}


pub enum Result {
    NoDataFound{ finish: Finish },
    DataFound{ data: std::string::String, finish: Finish },
}


pub struct String {
    error_source: error::Source,
    context: Context,
    string: std::string::String,
}


impl Into<crate::common::rustc::deserializer::Context> for Finish {
    fn into(self) -> crate::common::rustc::deserializer::Context {
        return match self {
            Finish::NoContext => crate::common::rustc::deserializer::Context::Freestanding,

            Finish::ArgumentEnd => crate::common::rustc::deserializer::Context::ArgumentEnd,
            Finish::ArgumentStart => crate::common::rustc::deserializer::Context::ArgumentStart,
            Finish::DataEnd => crate::common::rustc::deserializer::Context::DataEnd,
            Finish::DataStart => crate::common::rustc::deserializer::Context::DataStart,
            Finish::Freestanding => crate::common::rustc::deserializer::Context::Freestanding,
            Finish::ListEnd => crate::common::rustc::deserializer::Context::ListEnd,
            Finish::ListStart => crate::common::rustc::deserializer::Context::ListStart,
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
                self.context = Context::InitialComment;
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

            // Initial state context denominators
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::InitialArgumentEnd;
                block::AdvanceResult::Finished
            },
            (&Context::Initial,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::InitialArgumentStart;
                block::AdvanceResult::Finished
            },
            (&Context::Initial,
                DENOMINATOR_DATA_END) => {
                self.context = Context::InitialDataEnd;
                block::AdvanceResult::Finished
            },
            (&Context::Initial,
                DENOMINATOR_DATA_START) => {
                self.context = Context::InitialDataStart;
                block::AdvanceResult::Finished
            },
            (&Context::Initial,
                DENOMINATOR_LIST_END) => {
                self.context = Context::InitialListStart;
                block::AdvanceResult::Finished
            },
            (&Context::Initial,
                DENOMINATOR_LIST_START) => {
                self.context = Context::InitialListStart;
                block::AdvanceResult::Finished
            },
            (&Context::Initial,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::InitialSeparator;
                block::AdvanceResult::Finished
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

            (&Context::StringDenominated,
                DENOMINATOR_STRING) => {
                self.context = Context::FinishedFreestanding;
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
                self.context = Context::FinishedFreestanding;
                return block::AdvanceResult::Finished;
            },

            (&Context::StringFreestanding,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::FinishedArgumentEnd;
                return block::AdvanceResult::Finished;
            },
            (&Context::StringFreestanding,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedArgumentStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::StringFreestanding,
                DENOMINATOR_DATA_END) => {
                self.context = Context::FinishedDataEnd;
                return block::AdvanceResult::Finished;
            },
            (&Context::StringFreestanding,
                DENOMINATOR_DATA_START) => {
                self.context = Context::FinishedDataStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::StringFreestanding,
                DENOMINATOR_LIST_END) => {
                self.context = Context::FinishedListEnd;
                return block::AdvanceResult::Finished;
            },
            (&Context::StringFreestanding,
                DENOMINATOR_LIST_START) => {
                self.context = Context::FinishedListStart;
                return block::AdvanceResult::Finished;
            },
            (&Context::StringFreestanding,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::FinishedSeparator;
                return block::AdvanceResult::Finished;
            },

            (&Context::StringFreestanding,
                _) => {
                self.string.push(read_byte as char);
                return block::AdvanceResult::Continuous;
            },

            // On newline from comment reset deserializer
            (&Context::InitialComment,
                DENOMINATOR_NEW_LINE) => {
                self.context = Context::Initial;
                return block::AdvanceResult::Continuous;
            },

            // Ignore any other character in a comment
            (&Context::InitialComment,
                _) => {
                return block::AdvanceResult::Continuous;
            },

            (&Context::InitialArgumentEnd, _)
            | (&Context::InitialArgumentStart, _)
            | (&Context::InitialDataEnd, _)
            | (&Context::InitialDataStart, _)
            | (&Context::InitialListEnd, _)
            | (&Context::InitialListStart, _)
            | (&Context::InitialSeparator, _)
            | (&Context::FinishedArgumentEnd, _)
            | (&Context::FinishedArgumentStart, _)
            | (&Context::FinishedDataEnd, _)
            | (&Context::FinishedDataStart, _)
            | (&Context::FinishedFreestanding, _)
            | (&Context::FinishedListEnd, _)
            | (&Context::FinishedListStart, _)
            | (&Context::FinishedSeparator, _) => {
                return block::AdvanceResult::Finished;
            },

            (&Context::Error,
                _) => {
                return block::AdvanceResult::Error;
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Source> {
        return match self.context {
            Context::Initial
            | Context::InitialComment
            | Context::StringDenominated
            | Context::Error => {
                Err(self.error_source)
            },

            Context::InitialArgumentEnd => {
                Ok(Result::NoDataFound {
                    finish: Finish::ArgumentEnd,
                })
            },
            Context::InitialArgumentStart => {
                Ok(Result::NoDataFound {
                    finish: Finish::ArgumentStart,
                })
            },
            Context::InitialDataEnd => {
                Ok(Result::NoDataFound {
                    finish: Finish::DataEnd,
                })
            },
            Context::InitialDataStart => {
                Ok(Result::NoDataFound {
                    finish: Finish::DataStart,
                })
            },
            Context::InitialListEnd => {
                Ok(Result::NoDataFound {
                    finish: Finish::ListEnd,
                })
            },
            Context::InitialListStart => {
                Ok(Result::NoDataFound {
                    finish: Finish::ListStart,
                })
            },
            Context::InitialSeparator => {
                Ok(Result::NoDataFound {
                    finish: Finish::Separator,
                })
            },

            Context::StringFreestanding => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::NoContext,
                })
            },

            Context::FinishedArgumentEnd => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::ArgumentEnd,
                })
            },
            Context::FinishedArgumentStart => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::ArgumentStart,
                })
            },
            Context::FinishedDataEnd => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::DataEnd,
                })
            },
            Context::FinishedDataStart => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::DataStart,
                })
            },
            Context::FinishedFreestanding => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::Freestanding,
                })
            },
            Context::FinishedListEnd => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::ListEnd,
                })
            },
            Context::FinishedListStart => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::ListStart,
                })
            },
            Context::FinishedSeparator => {
                Ok(Result::DataFound {
                    data: self.string,
                    finish: Finish::Separator,
                })
            },
        };
    }
}