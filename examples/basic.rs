use rgparse::{Parser, Parameter};

fn main() {
    let mut parser = Parser::new("An example argument parser");
    parser.add_parameter(Parameter::param("--arg0", "Argument 0, type: u32."));
    parser.add_parameter(Parameter::param("--arg1", "Argument 1, type: f32.").alias("-a"));
    parser.add_parameter(Parameter::flag("--arg2", "Argument 2, flag").alias("-a2"));

    let args = parser.parse_args();
    for positional in &args.positional {
        println!("Found positional argument: {}", positional);
    }

    if let Some(arg0) = args.get::<u32>("--arg0") {
        println!("Arg 0 was {}", arg0);
    }

    if let Some(arg1) = args.get::<f32>("--arg1") {
        println!("Arg 1 was {}", arg1);
    }

    if args.flag("--arg2") {
        println!("Found Arg 2 flag!");
    }
}
