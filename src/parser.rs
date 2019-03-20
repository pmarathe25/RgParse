use std::str::FromStr;
use std::collections::HashMap;
use std::env;

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn can_create_parser() {
        let _parser = Parser::new("");
    }

    #[test]
    fn can_help() {
        let mut parser = Parser::new("Test Parser 0");
        parser.add_argument_short("-t", "--test", "A test argument");
        parser.help();
    }

    #[test]
    fn can_parse() {
        let mut parser = Parser::new("Test Parser 0");
        parser.add_argument_short("-t", "--test", "A test argument");
        println!("{:?}", parser.parse_args());
    }
}

// The arguments returned by parse.
#[derive(Debug)]
pub struct Args {
    pub positional: Vec<String>,
    arguments: HashMap<String, String>,
}

impl Args {
    fn new(positional: Vec<String>, arguments: HashMap<String, String>) -> Args {
        return Args{positional: positional, arguments: arguments};
    }

    pub fn get<T>(&self, arg: &str) -> Option<T> where T: FromStr {
        match self.arguments.get(arg) {
            Some(value) => {
                match value.parse::<T>() {
                    Ok(parsed) => return Some(parsed),
                    Err(_) => return None,
                }
            },
            None => return None,
        }
    }
}

pub struct Parser {
    // Description of the parser.
    description: String,
    // Maps long argument names to their descriptions.
    long_args: HashMap<String, String>,
    // Maps short argument aliases to their long arguments.
    short_args: HashMap<String, String>,
}

impl Parser {
    pub fn new(description: &str) -> Parser {
        return Parser{description: String::from(description), long_args: HashMap::new(), short_args: HashMap::new()};
    }

    pub fn add_argument_short(&mut self, short: &str, long: &str, description: &str) {
        self.add_argument(long, description);
        self.short_args.insert(String::from(short), String::from(long));
    }

    pub fn add_argument(&mut self, long: &str, description: &str) {
        self.long_args.insert(String::from(long), String::from(description));
    }

    pub fn help(&self) {
        // TODO: Print arguments in usage message.
        println!("{}\nUsage: {}", self.description, env::current_exe().unwrap().to_str().unwrap());
        // Create an inverse mapping of long argument names to short ones.
        let mut long_arg_aliases = HashMap::new();
        for (key, value) in &self.short_args {
            long_arg_aliases.insert(value, key);
        }
        // Display help messages for each argument.
        for (long_arg, description) in &self.long_args {
            let mut print_arg = String::from("\t");
            if let Some(short) = long_arg_aliases.get(long_arg) {
                print_arg += &format!("{}, ", short);
            }
            print_arg += long_arg;
            println!("{}\t{}", print_arg, description);
        }
    }

    pub fn parse_args(&self) -> Args {
        let mut positional = Vec::new();
        let mut arguments = HashMap::new();
        // Skip over the executable name.
        let mut args_iter = env::args().into_iter().skip(1);
        while let Some(arg) = args_iter.next() {
            if arg.starts_with("-") {
                let mut arg_name = arg.clone();
                let mut value;
                // Support both --arg=value and --arg value
                if arg.contains("=") {
                    let mut split = arg.splitn(2, "=");
                    arg_name = String::from(split.next()
                        .expect(&format!("Invalid argument: {}", arg))
                    );
                    value = String::from(split.next()
                        .expect(&format!("Invalid value to argument: {}", arg))
                    );
                } else {
                    value = args_iter.next()
                        .expect(&format!("Invalid value after argument: {}", arg));
                }
                // Convert short argument names to long ones.
                if let Some(long_arg) = self.short_args.get(&arg_name) {
                    arg_name = long_arg.clone();
                }
                if !self.long_args.contains_key(&arg_name) {
                    println!("Error: Unrecognized argument: {}", arg_name);
                    self.help();
                    std::process::exit(1);
                }
                arguments.insert(arg_name, value);
            } else {
                positional.push(arg);
            }
        }
        return Args::new(positional, arguments);
    }
}
