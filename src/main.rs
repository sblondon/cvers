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

    let version_a = &args[1];
    let version_b = &args[2];
    println!("{}", display::display(
            compare::compare(version_a, version_b)
        )
    );
}
