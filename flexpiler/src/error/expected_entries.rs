// intern
use crate::parser;
use std::fmt::{Pointer, Debug, Formatter};
use std::num::ParseIntError;


pub struct ExpectedEntries<EntryType> {
    vec_impl: std::vec::Vec<EntryType>,
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
        representation.push_str("}");
        write!(f, "{}", representation)
    }
}
