use std::env;
use std::process;

mod compare;
mod display;
mod errors;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        errors::exit_on_error("Invalid parameters");
    }

    let verb = &args[1];
    if verb == "compare" {
        main_compare(&args[2], &args[3]);
    } else if verb == "assert" {
        process::exit(
            main_assert(&args[2], &args[4], &args[3])
        );
    } else {
        let error_message = format!("Invalid verb '{}'. Use 'compare' or 'assert'.", verb);
        errors::exit_on_error(error_message.as_str());
    }
}

fn main_compare(version_a: &str, version_b: &str) {
    println!("{}", display::display(
            compare::compare(version_a, version_b)
        )
    );
}

fn main_assert(version_a: &str, version_b: &str, operator: &str) -> i32 {
    let order = compare::compare_with_operator(
        version_a, version_b, operator);
    match order {
        true => 0,
        false => 1,
    }
}
