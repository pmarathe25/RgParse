# RgParse - A Rust Command-line Argument Parser

A very simple command line argument parser.

## A Simple Example
```rust
// The argument to `new` is the description for the application.
let mut parser = Parser::new("An example argument parsing application");
// Adds some `param` parameters, which take a single value.
// There is no special requirement for parameter prefixes. `-/--` are used here by convention.
// Aliases and default values can be specified using the builder pattern.
parser.add_parameter(Parameter::param("--arg0", "Argument 0, type: u32.").alias("-a").default(&5));
// Adds a flag parameter, which takes no value - it is `true` if present, `false` otherwise
parser.add_parameter(Parameter::flag("--arg1", "Argument 1, flag").alias("-a1"));

// Parsing arguments returns an `Args` struct which has 3 major features.
let args = parser.parse_args();

// 1. You can access positional arguments directly.
for positional in &args.positional {
    println!("Found positional argument: {}", positional);
}

// 2. The `get` function allows you to access parameters.
let arg0 = args.get::<u32>("--arg0");
println!("Arg 0 was {}", arg0);

// 3. The `flag` function allows you to check the value of flags.
if args.flag("--arg2") {
    println!("Found Arg 2 flag!");
}
```
