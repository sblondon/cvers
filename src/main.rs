use std::env;
use std::process;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid parameters");
        process::exit(1);
    }

    let version_a = &args[1];
    let version_b = &args[2];
    compare(version_a.to_string(), version_b.to_string())
}

fn compare(version_a: String, version_b: String){
    println!("Version A {}", version_a);
    println!("Version B {}", version_b);
}
