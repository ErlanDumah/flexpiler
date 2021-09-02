pub mod expected_entries;
pub use expected_entries::ExpectedEntries;
pub mod propagation;
pub use propagation::Propagation;
pub mod stack_trace;
pub use stack_trace::StackTrace;
pub mod source;

// intern
use crate::parser;
use std::fmt::{Pointer, Debug, Formatter};
use std::num::ParseIntError;


pub struct Context {
    pub trace: String,
}


pub trait Trait<SourceType> : propagation::Trait<SourceType> {
    fn gen(source_type: impl Into<SourceType>) -> Propagation<SourceType>;
}


pub struct Error<SourceType> {
    pub stack_trace: StackTrace,
    pub error_source: SourceType,
}


impl<SourceType> std::fmt::Display for Error<SourceType>
where SourceType: std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error at {}: {}",
            self.stack_trace,
            self.error_source)
    }
}


impl<SourceType> propagation::Trait<SourceType> for Error<SourceType> {
    fn propagate(self, context: Context) -> Self {
        use stack_trace::Trait;

        let mut stack_trace = self.stack_trace;
        stack_trace.push(context.trace);
        let error_source = self.error_source;
        Error {
            stack_trace,
            error_source
        }
    }
}


impl<SourceType> Trait<SourceType> for Error<SourceType> {
    fn gen(source_type: impl Into<SourceType>) -> Propagation<SourceType> {
        Propagation::new(source_type.into())
    }
}
