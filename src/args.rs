use std::fmt::Debug;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

// The arguments returned by parse.
#[derive(Debug)]
pub struct Args {
    pub positional: Vec<String>,
    // Maps parameter names to values.
    pub(crate) arguments: HashMap<String, String>,
    pub(crate) flags: HashSet<String>,
}

impl Args {
    pub(crate) fn new() -> Args {
        return Args{positional: Vec::new(), arguments: HashMap::new(), flags: HashSet::new()};
    }

    pub fn flag(&self, flag: &str) -> bool {
        return self.flags.contains(flag);
    }

    pub fn get<T>(&self, param: &str) -> Option<T> where T: FromStr, <T as FromStr>::Err: Debug {
        match self.arguments.get(param) {
            Some(value) => {
                return Some(value.parse::<T>().expect(
                    &format!("Could not covert value ({}) for parameter {}", value, param)
                ));
            },
            None => return None,
        }
    }
}
