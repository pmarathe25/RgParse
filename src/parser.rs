use std::fmt::Debug;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::env;

#[cfg(test)]
mod tests {
    use super::{Parser, Parameter};

    #[test]
    fn can_create_parser() {
        let _parser = Parser::new("");
    }

    #[test]
    fn can_help() {
        let mut parser = Parser::new("Test Parser 0");
        parser.add_parameter(Parameter::param("--test", "A test parameter").alias("-t"));
        parser.help();
    }

    #[test]
    fn can_parse() {
        let mut parser = Parser::new("Test Parser 0");
        parser.add_parameter(Parameter::param("--test", "A test parameter").alias("-t"));
        println!("{:?}", parser.parse_args());
    }
}

// TODO: Move Args/Parameter into separate files.
// The arguments returned by parse.
#[derive(Debug)]
pub struct Args {
    pub positional: Vec<String>,
    // Maps parameter names to values.
    arguments: HashMap<String, String>,
    flags: HashSet<String>,
}

impl Args {
    fn new() -> Args {
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

#[derive(Debug)]
pub struct Parameter {
    name: String,
    takes_value: bool,
    description: String,
    alias: Option<String>,
}

impl Parameter {
    // TODO: Docstrings here
    /// An parameter that takes a value.
    /// Parameters do not have to start with - or --. This is up to the discretion of the user of this library..
    pub fn param(name: &str, description: &str) -> Parameter {
        return Parameter{name: String::from(name), takes_value: true, description: String::from(description), alias: None};
    }

    pub fn flag(name: &str, description: &str) -> Parameter {
        return Parameter{name: String::from(name), takes_value: false, description: String::from(description), alias: None};
    }

    pub fn alias(mut self, alias: &str) -> Parameter {
        self.alias = Some(String::from(alias));
        return self;
    }
}

// Internal information about the parameter.
#[derive(Debug)]
struct ParamInfo {
    takes_value: bool,
    description: String,
}

impl ParamInfo {
    pub fn new(takes_value: bool, description: String) -> ParamInfo {
        return ParamInfo{takes_value: takes_value, description: description};
    }
}

#[derive(Debug)]
pub struct Parser {
    // Description of the parser.
    description: String,
    // Maps long parameter names to their descriptions.
    parameters: HashMap<String, ParamInfo>,
    // Maps parameter aliases to their parameter names.
    aliases: HashMap<String, String>,
}

impl Parser {
    pub fn new(description: &str) -> Parser {
        return Parser{description: String::from(description), parameters: HashMap::new(), aliases: HashMap::new()};
    }

    pub fn add_parameter(&mut self, param: Parameter) {
        if let Some(alias) = param.alias {
            self.aliases.insert(alias, param.name.clone());
        }
        self.parameters.insert(param.name, ParamInfo::new(param.takes_value, param.description));
    }

    pub fn help(&self) {
        // TODO: Print parameters in usage message.
        println!("{}\nUsage: {}", self.description, env::current_exe().unwrap().to_str().unwrap());
        // Create an inverse mapping of long parameter names to short ones.
        let mut parameter_aliases = HashMap::new();
        for (key, value) in &self.aliases {
            parameter_aliases.insert(value, key);
        }
        // Display help messages for each parameter.
        for (param, info) in &self.parameters {
            let mut print_arg = String::from("\t");
            if let Some(short) = parameter_aliases.get(param) {
                print_arg += &format!("{}, ", short);
            }
            print_arg += param;
            println!("{}\t{}", print_arg, info.description);
        }
    }

    pub fn parse_args(&self) -> Args {
        fn fail(parser: &Parser) -> ! {
            parser.help();
            std::process::exit(1);
        }

        let mut args = Args::new();
        // Skip over the executable name.
        let mut args_iter = env::args().into_iter().skip(1);
        while let Some(arg) = args_iter.next() {
            // First check if this parameter is an alias. If so, get the full parameter name.
            let mut full_arg = arg.clone();
            if let Some(full_name) = self.aliases.get(&arg) {
                full_arg = full_name.clone();
            }

            if let Some(info) = self.parameters.get(&full_arg) {
                // If the argument was found, we will add it to the Args struct.
                if info.takes_value {
                    match args_iter.next() {
                        // If the value for this argument is invalid, print help message and exit.
                        Some(value) => args.arguments.insert(full_arg, value),
                        None => fail(&self),
                    };
                } else {
                    args.flags.insert(full_arg);
                }
            } else if arg == "-h" || arg == "--help" {
                // If -h/--help has not been overriden in the parser (i.e. was not found in the
                // if condition.), then we print and exit here.
                self.help();
                std::process::exit(0);
            } else {
                // Otherwise we have to assume that this is a positional argument.
                args.positional.push(arg);
            }
        }
        return args;
    }
}
