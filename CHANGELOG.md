# RgParse - A Rust Command-line Argument Parser
**NOTE**: Dates are in dd-mm-yyyy format.

## v0.1.0 (20-03-2019)
- Adds the Parser struct which holds arguments and descriptions
- Abandons support for `--argument=value` syntax. Only `--argument value` syntax is supported now. This greatly simplifies things.
- Introduces the Parameter struct to capture all relevant information about parameters
- Adds `param`, `flag` and `alias` functions to the Parameter struct.
