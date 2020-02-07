use std::env;
use std::process;
use std::fmt::Debug;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid parameters");
        process::exit(1);
    }

    let version_a = &args[1];
    let version_b = &args[2];
    println!("{}", display(
            compare(version_a.to_string(), version_b.to_string())
        )
    );
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Comparison {
    INF,
    EQU,
    SUP
}

fn compare(version_a: String, version_b: String)-> Comparison{
    let v_a: i32 = version_a.parse().unwrap();
    let v_b: i32 = version_b.parse().unwrap();
    if v_a > v_b {
        return Comparison::SUP;
    } else if v_a == v_b {
        return Comparison::EQU;
    } else {
        return Comparison::INF;
    }

}

fn display(comparison: Comparison)-> String{
    match comparison {
        Comparison::INF => {return "<".to_string();},
        Comparison::EQU => {return "=".to_string();},
        Comparison::SUP => {return ">".to_string();}
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_compare_equal() {
        assert_eq!(compare("2".to_string(), "2".to_string()), Comparison::EQU);
    }
    #[test]
    fn test_compare_inf() {
        assert_eq!(compare("3".to_string(), "2".to_string()), Comparison::SUP);
    }
    #[test]
    fn test_compare_sup() {
        assert_eq!(compare("2".to_string(), "3".to_string()), Comparison::INF);
    }

    #[test]
    fn test_display_inf() {
        assert_eq!(display(Comparison::INF), "<".to_string());
    }
    #[test]
    fn test_display_equal() {
        assert_eq!(display(Comparison::EQU), "=".to_string());
    }
    #[test]
    fn test_display_sup() {
        assert_eq!(display(Comparison::SUP), ">".to_string());
    }
}
