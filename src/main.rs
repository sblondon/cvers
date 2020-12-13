use std::env;
use std::process;

mod compare;
mod display;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid parameters");
        process::exit(1);
    }

    main_compare(&args[1], &args[2]);
}

fn main_compare(version_a: &str, version_b: &str) {
    println!("{}", display::display(
            compare::compare(version_a, version_b)
        )
    );
}
