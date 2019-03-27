use std::env;
use std::collections::HashMap;

use super::parameter::Parameter;
use super::args::Args;

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

// Internal information about parameters.
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
    // Keeps track of default arguments
    default_args: Args,
}

impl Parser {
    pub fn new(description: &str) -> Parser {
        return Parser{description: String::from(description), parameters: HashMap::new(), aliases: HashMap::new(), default_args: Args::new()};
    }

    pub fn add_parameter(&mut self, param: Parameter) {
        if let Some(alias) = param.alias {
            self.aliases.insert(alias, param.name.clone());
        }
        // For non-flags, insert the name into default_args.arguments.
        // These will be checked during parsing to ensure that they are not None.
        if param.takes_value {
            self.default_args.arguments.insert(param.name.clone(), param.default);
        }
        self.parameters.insert(param.name, ParamInfo::new(param.takes_value, param.description));
    }

    pub fn help(&self) {
        // Create an inverse mapping of long parameter names to short ones.
        let mut parameter_aliases = HashMap::new();
        for (key, value) in &self.aliases {
            parameter_aliases.insert(value, key);
        }
        // Usage message will just print parameter names.
        let mut usage = format!("{}\nUsage: {}", self.description, env::current_exe().unwrap().to_str().unwrap());
        for (mut param, _) in &self.parameters {
            let metavar = String::from(param.to_uppercase().trim_start_matches("-"));
            if let Some(short) = parameter_aliases.get(param) {
                param = short;
            }
            usage += &format!(" [{} {}]", param, metavar);
        }
        println!("{}", usage);
        // Full help message also includes descriptions for each parameter.
        for (param, info) in &self.parameters {
            let mut print_arg = String::from("\t");
            if let Some(short) = parameter_aliases.get(param) {
                print_arg += &format!("{},", short);
            }
            print_arg += &format!("\t{}", param);
            println!("{}\t\t{}", print_arg, info.description);
        }
    }

    pub fn parse_args(&self) -> Args {
        fn fail(parser: &Parser) -> ! {
            parser.help();
            std::process::exit(1);
        }

        let mut args = self.default_args.clone();
        // Skip over the executable name.
        let mut args_iter = env::args().into_iter().skip(1);
        while let Some(arg) = args_iter.next() {
            // First check if this parameter is an alias. If so, get the full parameter name.
            let mut full_arg = arg.clone();
            if let Some(full_name) = self.aliases.get(&arg) {
                full_arg = full_name.clone();
            }
            // If the argument was found, we will add it to the Args struct.
            if let Some(info) = self.parameters.get(&full_arg) {
                if info.takes_value {
                    match args_iter.next() {
                        // If the value for this argument is invalid, print help message and exit.
                        Some(value) => args.arguments.insert(full_arg, Some(value)),
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

        // All arguments should either have a default value, or a user provided one at this point.
        let mut missing_args = Vec::new();
        for (arg, value_opt) in &args.arguments {
            if value_opt.is_none() {
                missing_args.push(arg);
            }
        }
        if !missing_args.is_empty() {
            println!("Missing required arguments: {:?}", missing_args);
            fail(&self);
        }

        return args;
    }
}
