use std::fmt::{Formatter, Debug};

use crate::error::ExpectedEntries;


pub struct UnexpectedToken {
    pub token_expected_entries: ExpectedEntries<char>,
    pub token_found: char,
}


pub enum Source {
    NoContent, // No content has been parsed / could be parsed

    EndOfData,
    EndOfFile,

    IllegalDeclarationTokenFormat,

    UnexpectedDeclarationEmpty,
    UnexpectedStringEmpty,

    UnexpectedToken(UnexpectedToken),

    IOError(std::io::Error),
}


impl std::fmt::Display for UnexpectedToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected any of mandatory token {}, found token {}", self.token_expected_entries, self.token_found)
    }
}


impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Source::NoContent => {
                write!(f, "Content was expected but none was found")
            },

            &Source::EndOfData => {
                write!(f, "Data was abruptly ended when more content was expected")
            },
            &Source::EndOfFile => {
                write!(f, "File has abruptly ended when more content was expected")
            },
            &Source::IllegalDeclarationTokenFormat => {
                write!(f, "Formatting tokens are illegal for declarations.")
            }
            &Source::UnexpectedDeclarationEmpty => {
                write!(f, "Expected a non-empty declaration.")
            },
            &Source::UnexpectedStringEmpty => {
                write!(f, "Expected a non-empty string.")
            },
            &Source::UnexpectedToken(unexpected_token_ref) => {
                return unexpected_token_ref.fmt(f);
            },

            &Source::IOError(error_ref) => {
                return <std::io::Error as std::fmt::Display>::fmt(error_ref, f);
            }
        }
    }
}
