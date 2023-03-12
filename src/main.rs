use std::env;
use std::process;

use std::collections::HashSet;

mod args;
mod compare;
mod display;
mod errors;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        errors::exit_on_error("Missing parameters");
    } else if args[1] == "--help" {
        help();
    } else if args.len() < 4 {
        errors::exit_on_error("Missing parameters");
    } else {
        let tuple = args::parse_arguments(args);
        let config: compare::ParserConfig = tuple.0;
        let mandatories_args: Vec<String> = tuple.1;
        canonical_operations(config, mandatories_args);
    }
}

fn help() {
    println!("Usage:
 - cvers compare version1 version2
 - cvers assert version1 operator version2
 - cvers --help");
}

fn canonical_operations(parser_config: compare::ParserConfig, args: Vec<String>) {
    let verb = args[1].as_str();
    match verb {
        "compare" => {
            compare_operation(&args[2], &args[3], parser_config);
        },
        "assert" => {
            let operator = &args[3];
            let operators: HashSet<&'static str> = ["<<", "<=", "==", "=>", ">>", "!="].iter().cloned().collect();
            if ! operators.contains(&operator.as_str()) {
                let error_message = format!("Invalid operator '{operator}'.", operator=operator);
                errors::exit_on_error(error_message.as_str());
            } else {
                process::exit(
                    assert_operation(&args[2], &args[4], operator, parser_config)
                );
            }
        },
        _ => {
            let error_message = format!("Invalid verb '{verb}'. Use 'compare' or 'assert'.", verb=verb);
            errors::exit_on_error(error_message.as_str());
        }
    }
}

fn compare_operation(version_a: &str, version_b: &str, parser_config: compare::ParserConfig) {
    println!("{}", display::display(
            compare::compare(version_a, version_b, &parser_config)
        )
    );
}

fn assert_operation(version_a: &str, version_b: &str, operator: &str, parser_config: compare::ParserConfig) -> i32 {
    let order = compare::compare_with_operator(
        version_a, version_b, operator, &parser_config);
    match order {
        true => 0,
        false => 1,
    }
}
