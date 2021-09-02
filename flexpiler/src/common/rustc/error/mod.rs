use crate::error;
use crate::parser;
use crate::error::source::Common;


pub struct IncompatibleEnumDeclaration {
    pub enum_declaration_expected_entries: error::ExpectedEntries<String>,
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


pub struct MissingEnumArgument {
    pub enum_declaration_found: std::string::String,
    pub argument_type_expected: std::string::String,
}


pub struct MissingEnumComplexField {
    pub enum_declaration_found: std::string::String,
    pub field_declaration_expected: std::string::String,
}


pub struct MissingStructField {
    pub struct_declaration_found: std::string::String,
    pub field_declaration_expected: std::string::String,
}


pub struct UnexpectedContext {
    pub context_found: crate::common::rustc::deserializer::Context,
    pub context_expected: error::ExpectedEntries<crate::common::rustc::deserializer::Context>,
}


pub struct UnexpectedEntryFinishContext {
    pub entry_declaration: std::string::String,
    pub context_found: crate::common::rustc::deserializer::Context,
    pub context_expected: error::ExpectedEntries<crate::common::rustc::deserializer::Context>,
}


pub struct UnrecognizedFieldDeclaration {
    pub field_declaration_found: std::string::String,
    pub field_declaration_expected_entries: error::ExpectedEntries<String>,
}


pub enum Source {
    Parser(parser::error::Source),
    ConversionParseIntError(core::num::ParseIntError),
    Common(error::source::Common),

    IncompatibleEnumDeclaration(IncompatibleEnumDeclaration),
    IncompatibleEnumDataType(IncompatibleEnumDataType),
    IncompatibleStructDeclaration(IncompatibleStructDeclaration),
    MissingEnumArgumentSeparator(MissingEnumArgumentSeparator),
    MissingEnumArgument(MissingEnumArgument),
    MissingEnumArgumentClosure(MissingEnumArgumentClosure),
    MissingEnumComplexField(MissingEnumComplexField),
    MissingStructField(MissingStructField),
    UnexpectedContext(UnexpectedContext),
    UnexpectedEntryFinishContext(UnexpectedEntryFinishContext),
    UnrecognizedFieldDeclaration(UnrecognizedFieldDeclaration),
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


impl std::fmt::Display for MissingEnumArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enum declaration of type {} is missing an argument of type {}",
               self.enum_declaration_found,
               self.argument_type_expected)
    }
}


impl std::fmt::Display for MissingEnumArgumentClosure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enum declaration of type {} is missing an argument closure",
               self.enum_declaration_found)
    }
}


impl std::fmt::Display for MissingEnumArgumentSeparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Enum declaration of type {} is missing an argument separator",
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


impl std::fmt::Display for UnexpectedContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected context: found a {} context unexpectedly. Expected {}.",
               self.context_found,
               self.context_expected)
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


impl std::fmt::Display for UnrecognizedFieldDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unexpected field declaration: Found a field of name {} unexpectedly. Expected {}.",
               self.field_declaration_found,
               self.field_declaration_expected_entries)
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
            &Source::Common(ref error) => {
                return error.fmt(f);
            }
            &Source::IncompatibleStructDeclaration(ref incompatible_struct_declaration_ref) => {
                return incompatible_struct_declaration_ref.fmt(f);
            }
            &Source::IncompatibleEnumDataType(ref incompatible_enum_data_type_ref) => {
                return incompatible_enum_data_type_ref.fmt(f);
            }
            &Source::IncompatibleEnumDeclaration(ref incompatible_enum_declaration_ref) => {
                return incompatible_enum_declaration_ref.fmt(f);
            }
            &Source::MissingEnumArgument(ref missing_enum_argument_ref) => {
                return missing_enum_argument_ref.fmt(f);
            },
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
            &Source::UnexpectedContext(ref unexpected_context_ref) => {
                return unexpected_context_ref.fmt(f);
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


impl Into<Source> for MissingEnumArgument {
    fn into(self) -> Source { Source::MissingEnumArgument(self) }
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


impl Into<Source> for UnexpectedContext {
    fn into(self) -> Source {
        Source::UnexpectedContext(self)
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


impl From<parser::error::Source> for Source {
    fn from(parser_error_source: parser::error::Source) -> Self {
        Source::Parser(parser_error_source)
    }
}


impl From<core::num::ParseIntError> for Source {
    fn from(parse_int_error: core::num::ParseIntError) -> Self {
        Source::ConversionParseIntError(parse_int_error)
    }
}


impl From<error::source::Common> for Source {
    fn from(common_error: Common) -> Self {
        Source::Common(common_error)
    }
}


impl error::source::Trait for Source {
}
