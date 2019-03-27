# RgParse - A Rust Command-line Argument Parser
**NOTE**: Dates are in dd-mm-yyyy format.

## vNext
- Adds support for default values for parameters with the `default` function.
- Adds parameter names to `Usage` section in help message.
- Adds checks in `parse_args` to ensure that all required arguments have been supplied.

## v0.1.1 (20-03-2019)
- Adds a basic example to the README.
- Restructures parser.rs - now split into `parameter.rs`, `args.rs`, and `parser.rs`.

## v0.1.0 (20-03-2019)
- Adds the Parser struct which holds arguments and descriptions
- Abandons support for `--argument=value` syntax. Only `--argument value` syntax is supported now. This greatly simplifies things.
- Introduces the Parameter struct to capture all relevant information about parameters
- Adds `param`, `flag` and `alias` functions to the Parameter struct.
