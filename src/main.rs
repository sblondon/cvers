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
    let v_a: i32 = version_a.parse().unwrap();
    let v_b: i32 = version_b.parse().unwrap();
    if v_a > v_b {
        println!("A {} > B {}", version_a, version_b);
    } else if v_a == v_b {
        println!("A {} == B {}", version_a, version_b);
    } else {
        println!("A {} < B {}", version_a, version_b);
    }
}
