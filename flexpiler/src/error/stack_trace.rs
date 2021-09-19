

pub trait Trait {
    fn push(&mut self, value: String);
}


pub struct StackTrace {
    vec_impl: std::vec::Vec<std::string::String>,
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


impl Trait for StackTrace {
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
