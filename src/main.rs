use std::env;
use std::process;

mod compare;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid parameters");
        process::exit(1);
    }

    let version_a = &args[1];
    let version_b = &args[2];
    println!("{}", compare::display(
            compare::compare(version_a.to_string(), version_b.to_string())
        )
    );
}
