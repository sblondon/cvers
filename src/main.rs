use std::env;
use std::process;

use std::collections::HashSet;

mod compare;
mod display;
mod errors;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        errors::exit_on_error("Invalid parameters");
    }

    let parser_config = compare::ParserConfig {
        epoch_separator: Some(':'),
        pre_release_touchs_digit: None,
    };
    let verb = args[1].as_str();
    match verb {
        "compare" => {
            main_compare(&args[2], &args[3], parser_config);
        },
        "assert" => {
            let operator = &args[3];
            let operators: HashSet<&'static str> = ["<<", "<=", "==", "=>", ">>"].iter().cloned().collect();
            if ! operators.contains(&operator.as_str()) {
                let error_message = format!("Invalid operator '{operator}'.", operator=operator);
                errors::exit_on_error(error_message.as_str());
            } else {
                process::exit(
                    main_assert(&args[2], &args[4], operator, parser_config)
                );
            }
        },
        _ => {
            let error_message = format!("Invalid verb '{verb}'. Use 'compare' or 'assert'.", verb=verb);
            errors::exit_on_error(error_message.as_str());
        }
    }
}

fn main_compare(version_a: &str, version_b: &str, parser_config: compare::ParserConfig) {
    println!("{}", display::display(
            compare::compare(version_a, version_b, &parser_config)
        )
    );
}

fn main_assert(version_a: &str, version_b: &str, operator: &str, parser_config: compare::ParserConfig) -> i32 {
    let order = compare::compare_with_operator(
        version_a, version_b, operator, &parser_config);
    match order {
        true => 0,
        false => 1,
    }
}
