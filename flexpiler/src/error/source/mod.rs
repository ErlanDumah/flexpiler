use crate::parser;

pub mod common;
pub use common::Common;


pub trait Trait: From<parser::error::Source> +
                 From<core::num::ParseIntError> +
                 From<Common>
{
}


/*
pub enum Source {
    Parser(parser::error::Source),
    ConversionParseIntError(core::num::ParseIntError),
    Common(Common),

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
 */
