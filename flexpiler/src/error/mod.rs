// intern
use crate::parser;
use std::fmt::{Pointer, Debug, Formatter};
use std::num::ParseIntError;


pub struct ExpectedEntries<EntryType> {
    vec_impl: std::vec::Vec<EntryType>,
}


pub struct StackTrace {
    vec_impl: std::vec::Vec<std::string::String>,
}


pub struct IncompatibleEnumDeclaration {
    pub enum_declaration_expected_entries: ExpectedEntries<String>,
    pub enum_declaration_found: std::string::String,
}


pub mod incompatibleenumdataType {
    pub enum EnumDataType {
        NoData,
        Argument,
        Complex,
    }
}


pub struct IncompatibleEnumDataType {
    pub enum_declaration_found: std::string::String,
    pub enum_data_type_expected: incompatibleenumdataType::EnumDataType,
    pub context_found: crate::common::rustc::deserializer::Context,
}


pub struct IncompatibleStructDeclaration {
    pub struct_declaration_expected: std::string::String,
    pub struct_declaration_found: std::string::String,
}


pub struct MissingEnumArgumentSeparator {
    pub enum_declaration_found: std::string::String,
}


pub struct MissingEnumArgumentClosure {
    pub enum_declaration_found: std::string::String,
}


pub struct MissingEnumComplexField {
    pub enum_declaration_found: std::string::String,
    pub field_declaration_expected: std::string::String,
}


pub struct MissingStructField {
    pub struct_declaration_found: std::string::String,
    pub field_declaration_expected: std::string::String,
}


pub struct UnexpectedEntryFinishContext {
    pub entry_declaration: std::string::String,
    pub context_found: crate::common::rustc::deserializer::Context,
    pub context_expected: ExpectedEntries<crate::common::rustc::deserializer::Context>,
}


pub struct UnrecognizedFieldDeclaration {
    pub field_declaration_found: std::string::String,
    pub field_declaration_expected_entries: ExpectedEntries<String>,
}


pub enum Source {
    Parser(parser::error::Source),
    ConversionParseIntError(core::num::ParseIntError),

    IncompatibleEnumDeclaration(IncompatibleEnumDeclaration),
    IncompatibleEnumDataType(IncompatibleEnumDataType),
    IncompatibleStructDeclaration(IncompatibleStructDeclaration),
    MissingEnumArgumentSeparator(MissingEnumArgumentSeparator),
    MissingEnumArgumentClosure(MissingEnumArgumentClosure),
    MissingEnumComplexField(MissingEnumComplexField),
    MissingStructField(MissingStructField),
    UnexpectedEntryFinishContext(UnexpectedEntryFinishContext),
    UnrecognizedFieldDeclaration(UnrecognizedFieldDeclaration),
}


pub trait StackTraceTrait {
    fn push(&mut self, value: String);
}


pub struct Context {
    pub trace: String,
}


/**
 * An error that is yet to be put in perspective with its context.
 *
 * Error instances are made from their Source and then propagated through one or multiple Context instances.
 */
pub struct Propagation {
    error_source: Source,
}


pub struct Error {
    pub stack_trace: StackTrace,
    pub error_source: Source,
}


impl Error {
    pub fn gen(source_type: impl Into<Source>) -> Propagation {
        Propagation {
            error_source: source_type.into(),
        }
    }
}


pub trait Trait {
    fn propagate(self, context: Context) -> Error;
}


impl StackTrace {
    pub fn new() -> StackTrace {
        StackTrace {
            vec_impl: std::vec::Vec::new(),
        }
    }
}


impl From<std::vec::Vec<String>> for StackTrace {
    fn from(vec_impl: Vec<String>) -> Self {
        StackTrace {
            vec_impl
        }
    }
}


impl StackTraceTrait for StackTrace {
    fn push(&mut self, value: String) {
        self.vec_impl.push(value);
    }
}


impl std::fmt::Display for StackTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.vec_impl.iter().rev();
        let mut representation = match iter.next() {
            Some(value) => value.clone(),
            None => String::from("<root>"),
        };
        // Add remaining values with a '::' in front
        for entry_ref in iter {
            representation.push_str("::");
            representation.push_str(entry_ref.as_str());
        }
        write!(f, "{}", representation)
    }
}


impl<EntryType> From<std::vec::Vec<EntryType>> for ExpectedEntries<EntryType> {
    fn from(vec_impl: Vec<EntryType>) -> Self {
        ExpectedEntries {
            vec_impl
        }
    }
}


impl<EntryType> std::fmt::Display for ExpectedEntries<EntryType>
where EntryType: std::fmt::Display{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut iter = self.vec_impl.iter();
        let mut representation = String::from("{");
        match iter.next() {
            Some(value) => representation.push_str(format!("{}", value).as_str()),
            None => {},
        };
        // Add remaining values with a '::' in front
        for entry_ref in iter {
            representation.push_str(", ");
            representation.push_str(format!("{}", entry_ref).as_str());
        }
        write!(f, "{}", representation)
    }
}


impl std::fmt::Display for IncompatibleStructDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected declaration of struct as {}, instead found unexpected declaration name {}",
               self.struct_declaration_expected,
               self.struct_declaration_found)
    }
}


impl std::fmt::Display for IncompatibleEnumDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.enum_data_type_expected {
            &incompatibleenumdataType::EnumDataType::NoData => {
                write!(f, "Expected an enum declaration of format {}",
                    self.enum_declaration_found)
            },
            &incompatibleenumdataType::EnumDataType::Argument => {
                write!(f, "Expected an enum declaration of format {}()",
                       self.enum_declaration_found)
            },
            &incompatibleenumdataType::EnumDataType::Complex => {
                write!(f, "Expected an enum declaration of format {}{}",
                       self.enum_declaration_found,
                       "{}")
            },
        }
    }
}


impl std::fmt::Display for IncompatibleEnumDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Expected declaration of enum as any of {}, instead found unexpected declaration name {}",
               self.enum_declaration_expected_entries,
               self.enum_declaration_found)
    }
}


impl std::fmt::Display for MissingEnumArgumentClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enum declaration of type {} is missing an argument closure",
               self.enum_declaration_found)
    }
}


impl std::fmt::Display for MissingStructField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing content: struct declaration {} is lacking expected field {}.",
               self.struct_declaration_found,
               self.field_declaration_expected)
    }
}


impl std::fmt::Display for MissingEnumComplexField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Missing content: complex enum declaration {} is lacking expected field {}.",
               self.enum_declaration_found,
               self.field_declaration_expected)
    }
}


impl std::fmt::Display for UnexpectedEntryFinishContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected context: entry of name {} finished with a {} unexpectedly. Expected {}.",
               self.entry_declaration,
               self.context_found,
               self.context_expected)
    }
}


impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            &Source::Parser(ref parser_error_source) => {
                return parser_error_source.fmt(f);
            },
            &Source::ConversionParseIntError(ref error) => {
                return <core::num::ParseIntError as std::fmt::Display>::fmt(error, f);
            },
            &Source::IncompatibleStructDeclaration(ref incompatible_struct_declaration_ref) => {
                return incompatible_struct_declaration_ref.fmt(f);
            }
            &Source::IncompatibleEnumDataType(ref incompatible_enum_data_type_ref) => {
                return incompatible_enum_data_type_ref.fmt(f);
            }
            &Source::IncompatibleEnumDeclaration(ref incompatible_enum_declaration_ref) => {
                return incompatible_enum_declaration_ref.fmt(f);
            }
            &Source::MissingEnumArgumentClosure(ref missing_enum_argument_closure_ref) => {
                return missing_enum_argument_closure_ref.fmt(f);
            },
            &Source::MissingEnumArgumentSeparator(ref missing_enum_argument_separator_ref) => {
                return missing_enum_argument_separator_ref.fmt(f);
            }
            &Source::MissingEnumComplexField(ref missing_enum_complex_field_ref) => {
                return missing_enum_complex_field_ref.fmt(f);
            },
            &Source::MissingStructField(ref missing_struct_field_ref) => {
                return missing_struct_field_ref.fmt(f);
            },
            &Source::UnexpectedEntryFinishContext(ref unexpected_entry_finish_context_ref) => {
                return unexpected_entry_finish_context_ref.fmt(f);
            },
            &Source::UnrecognizedFieldDeclaration(ref unrecognized_field_declaration_ref) => {
                return unrecognized_field_declaration_ref.fmt(f);
            }
        }
    }
}


impl Trait for Propagation {
    fn propagate(self, context: Context) -> Error {
        let error = Error {
            stack_trace: StackTrace::new(),
            error_source: self.error_source,
        };
        return error.propagate(context);
    }
}


impl Into<Source> for IncompatibleEnumDataType {
    fn into(self) -> Source {
        Source::IncompatibleEnumDataType(self)
    }
}


impl Into<Source> for IncompatibleEnumDeclaration {
    fn into(self) -> Source {
        Source::IncompatibleEnumDeclaration(self)
    }
}


impl Into<Source> for IncompatibleStructDeclaration {
    fn into(self) -> Source {
        Source::IncompatibleStructDeclaration(self)
    }
}


impl Into<Source> for MissingEnumArgumentClosure {
    fn into(self) -> Source { Source::MissingEnumArgumentClosure(self) }
}


impl Into<Source> for MissingStructField {
    fn into(self) -> Source {
        Source::MissingStructField(self)
    }
}


impl Into<Source> for MissingEnumArgumentSeparator {
    fn into(self) -> Source {
        Source::MissingEnumArgumentSeparator(self)
    }
}


impl Into<Source> for MissingEnumComplexField {
    fn into(self) -> Source {
        Source::MissingEnumComplexField(self)
    }
}


impl Into<Source> for UnexpectedEntryFinishContext {
    fn into(self) -> Source {
        Source::UnexpectedEntryFinishContext(self)
    }
}


impl Into<Source> for UnrecognizedFieldDeclaration {
    fn into(self) -> Source {
        Source::UnrecognizedFieldDeclaration(self)
    }
}


impl From<parser::error::Error> for Source {
    fn from(parser_error: parser::error::Error) -> Self {
        Source::Parser(parser_error.error_source)
    }
}


impl From<core::num::ParseIntError> for Source {
    fn from(parse_int_error: core::num::ParseIntError) -> Self {
        Source::ConversionParseIntError(parse_int_error)
    }
}


impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}: {}",
            self.stack_trace,
            self.error_source)
    }
}


impl Trait for Error {
    fn propagate(self, context: Context) -> Error {
        let mut stack_trace = self.stack_trace;
        stack_trace.push(context.trace);
        let error_source = self.error_source;
        Error {
            stack_trace,
            error_source
        }
    }
}
