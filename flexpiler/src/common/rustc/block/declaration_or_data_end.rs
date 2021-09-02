use super::constants::*;

use crate::block;
use crate::parser::error;
use crate::reader;
use crate::error::ExpectedEntries;


enum DeclarationOrDataEndContext {
    Initial,

    Comment,

    DataEndFinished,

    String,
    StringFinished,

    Error,
}


pub struct DeclarationOrDataEnd {
    error_source: error::Source,
    context: DeclarationOrDataEndContext,
    string: std::string::String,
}


#[derive(Clone)]
pub enum Result {
    DataEnd(),
    Declaration(String),
}


impl DeclarationOrDataEnd {
    pub fn new() -> DeclarationOrDataEnd {
        DeclarationOrDataEnd {
            error_source: error::Source::NoContent,
            context: DeclarationOrDataEndContext::Initial,
            string: std::string::String::new(),
        }
    }
}


impl Default for DeclarationOrDataEnd {
    fn default() -> Self {
        return DeclarationOrDataEnd::new();
    }
}


impl block::Trait for DeclarationOrDataEnd {
    type Result = Result;

    fn advance_result(&mut self, read_byte: u8) -> block::AdvanceResult {
        match (&self.context, read_byte) {
            // Accept comments in initial state
            (&DeclarationOrDataEndContext::Initial,
                DENOMINATOR_COMMENT) => {
                self.context = DeclarationOrDataEndContext::Comment;
                return block::AdvanceResult::Continuous;
            },

            // Ignore newline, space, tab at initial state
            (&DeclarationOrDataEndContext::Initial,
                DENOMINATOR_NEW_LINE)
            | (&DeclarationOrDataEndContext::Initial,
                DENOMINATOR_SPACE)
            | (&DeclarationOrDataEndContext::Initial,
                DENOMINATOR_TAB) => {
                return block::AdvanceResult::Continuous;
            },

            // On encountering a declaration nominator as first character
            // Throw error for not allowed empty declaration
            (&DeclarationOrDataEndContext::Initial,
                DENOMINATOR_DECLARATION) => {
                self.error_source = error::Source::UnexpectedDeclarationEmpty;
                self.context = DeclarationOrDataEndContext::Error;
                return block::AdvanceResult::Error;
            },

            // On encountering a declaration nominator as first character
            // Throw error for not allowed empty declaration
            (&DeclarationOrDataEndContext::Initial,
                DENOMINATOR_DATA_END) => {
                self.context = DeclarationOrDataEndContext::DataEndFinished;
                return block::AdvanceResult::Finished;
            },

            // On encountering the first non ignorable | comment char
            // Change deserializer to string and add read_byte as first character
            (&DeclarationOrDataEndContext::Initial,
                _) => {
                self.string.push(read_byte as char);
                self.context = DeclarationOrDataEndContext::String;
                return block::AdvanceResult::Continuous;
            },

            // On encountering the first non ignorable | comment char
            // Change deserializer to string and add read_byte as first character
            (&DeclarationOrDataEndContext::DataEndFinished,
                _) => {
                return block::AdvanceResult::Finished;
            },

            (&DeclarationOrDataEndContext::String,
                DENOMINATOR_DECLARATION) => {
                self.context = DeclarationOrDataEndContext::StringFinished;
                return block::AdvanceResult::Finished;
            },

            (&DeclarationOrDataEndContext::String,
                DENOMINATOR_SPACE)
            | (&DeclarationOrDataEndContext::String,
                DENOMINATOR_NEW_LINE)
            | (&DeclarationOrDataEndContext::String,
                DENOMINATOR_TAB) => {
                self.error_source = error::Source::IllegalDeclarationTokenFormat;
                return block::AdvanceResult::Error;
            },

            // On encountering regular
            (&DeclarationOrDataEndContext::String,
                _) => {
                self.string.push(read_byte as char);
                return block::AdvanceResult::Continuous;
            },

            // On newline from comment reset deserializer
            (&DeclarationOrDataEndContext::Comment,
                DENOMINATOR_NEW_LINE) => {
                self.context = DeclarationOrDataEndContext::Initial;
                return block::AdvanceResult::Continuous;
            },

            // Ignore any other character in a comment
            (&DeclarationOrDataEndContext::Comment,
                _) => {
                return block::AdvanceResult::Continuous;
            },

            (&DeclarationOrDataEndContext::StringFinished,
                _) => {
                return block::AdvanceResult::Finished;
            },

            (&DeclarationOrDataEndContext::Error,
                _) => {
                return block::AdvanceResult::Error;
            },
        }
    }

    fn into_result(self) -> std::result::Result<Result, error::Source> {
        match self.context {
            DeclarationOrDataEndContext::Initial
            | DeclarationOrDataEndContext::Comment
            | DeclarationOrDataEndContext::String
            | DeclarationOrDataEndContext::Error => {
                return Err(self.error_source);
            }
            DeclarationOrDataEndContext::DataEndFinished => {
                return Ok(Result::DataEnd())
            },
            DeclarationOrDataEndContext::StringFinished => {
                return Ok(Result::Declaration(self.string))
            },
        }
    }
}
