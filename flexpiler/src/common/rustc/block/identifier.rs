use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;


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

    IdentifierString,

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


#[derive(Clone)]
pub enum Finish {
    // Block may be successfully parsed even if the end of data is reached
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


#[derive(Clone)]
pub enum Result {
    NoDataFound{ finish: Finish },
    DataFound{ data: std::string::String, finish: Finish },
}


pub struct Identifier {
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


impl std::fmt::Display for Finish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Finish::NoContext => {
                write!(f, "NoContext")
            },
            &Finish::ArgumentEnd => {
                write!(f, "ArgumentEnd")
            },
            &Finish::ArgumentStart => {
                write!(f, "ArgumentStart")
            },
            &Finish::DataEnd => {
                write!(f, "DataEnd")
            },
            &Finish::DataStart => {
                write!(f, "DataStart")
            },
            &Finish::Freestanding => {
                write!(f, "Freestanding")
            },
            &Finish::ListEnd => {
                write!(f, "ListEnd")
            },
            &Finish::ListStart => {
                write!(f, "ListStart")
            },
            &Finish::Separator => {
                write!(f, "Separator")
            },
        }
    }
}


impl Identifier {
    pub fn new() -> Identifier {
        Identifier {
            error_source: error::Source::NoContent,
            context: Context::Initial,
            string: std::string::String::new(),
        }
    }
}


impl Default for Identifier {
    fn default() -> Self {
        return Self::new();
    }
}


impl block::Trait for Identifier {
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
                self.context = Context::InitialListEnd;
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
                self.context = Context::FinishedFreestanding;
                block::AdvanceResult::Finished
            },

            // Identifier finished with context denominator
            (&Context::IdentifierString,
                DENOMINATOR_ARGUMENT_END) => {
                self.context = Context::FinishedArgumentEnd;
                block::AdvanceResult::Finished
            },
            (&Context::IdentifierString,
                DENOMINATOR_ARGUMENT_START) => {
                self.context = Context::FinishedArgumentStart;
                block::AdvanceResult::Finished
            },
            (&Context::IdentifierString,
                DENOMINATOR_DATA_END) => {
                self.context = Context::FinishedDataEnd;
                block::AdvanceResult::Finished
            },
            (&Context::IdentifierString,
                DENOMINATOR_DATA_START) => {
                self.context = Context::FinishedDataStart;
                block::AdvanceResult::Finished
            },
            (&Context::IdentifierString,
                DENOMINATOR_LIST_END) => {
                self.context = Context::FinishedListEnd;
                block::AdvanceResult::Finished
            },
            (&Context::IdentifierString,
                DENOMINATOR_LIST_START) => {
                self.context = Context::FinishedListStart;
                block::AdvanceResult::Finished
            },
            (&Context::IdentifierString,
                DENOMINATOR_SEPARATOR) => {
                self.context = Context::FinishedSeparator;
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
                block::AdvanceResult::Finished
            },

            (&Context::Error,
                _) => {
                block::AdvanceResult::Error
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Source> {
        return match self.context {
            Context::Initial
            | Context::InitialComment
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

            Context::IdentifierString => {
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
