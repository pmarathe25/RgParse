use std::fmt::Debug;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};

// The arguments returned by parse.
#[derive(Debug, Clone)]
pub struct Args {
    pub positional: Vec<String>,
    // Maps parameter names to values.
    pub(crate) arguments: HashMap<String, Option<String>>,
    pub(crate) flags: HashSet<String>,
}

impl Args {
    pub(crate) fn new() -> Args {
        return Args{positional: Vec::new(), arguments: HashMap::new(), flags: HashSet::new()};
    }

    pub fn flag(&self, flag: &str) -> bool {
        return self.flags.contains(flag);
    }

    // TODO: This should be in the parser, so that we can display a help message on conversion failure.
    pub fn get<T>(&self, param: &str) -> T where T: FromStr, <T as FromStr>::Err: Debug {
        match self.arguments.get(param) {
            Some(Some(value)) => {
                return value.parse::<T>().expect(
                    &format!("Could not covert value ({}) for parameter {}", value, param)
                );
            },
            None => panic!("Parameter {} unrecognized", param),
            // Unreachable because the parser ensures that every argument has some value.
            _ => unreachable!(),
        }
    }
}
