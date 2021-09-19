// intern
use crate::error;


pub trait Trait<SourceType> {
    fn propagate(self, context: error::Context) -> error::Error<SourceType>;
}


/**
 * An error that is yet to be put in perspective with its context.
 *
 * Error instances are made from their Source and then propagated through one or multiple Context instances.
 */
pub struct Propagation<SourceType> {
    error_source: SourceType,
}


impl<SourceType> Propagation<SourceType> {
    pub fn new(error_source: SourceType) -> Self {
        Propagation {
            error_source,
        }
    }
}


impl<SourceType> Trait<SourceType> for Propagation<SourceType> {
    fn propagate(self, context: error::Context) -> error::Error<SourceType> {
        use error::Trait;

        let error = error::Error {
            stack_trace: error::StackTrace::new(),
            error_source: self.error_source,
        };
        return error.propagate(context);
    }
}
