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
    println!("{}", display(
            compare(version_a.to_string(), version_b.to_string())
        )
    );
}

fn compare(version_a: String, version_b: String)-> i32{
    let v_a: i32 = version_a.parse().unwrap();
    let v_b: i32 = version_b.parse().unwrap();
    if v_a > v_b {
        return -1;
    } else if v_a == v_b {
        return 0;
    } else {
        return 1;
    }

}

fn display(comparison: i32)-> std::string::String{
    if comparison == 1 {
        return "<".to_string();
    } else if comparison == 0 {
        return "=".to_string();
    } else {
        return ">".to_string();
    }

}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_compare_equal() {
        assert_eq!(compare("2".to_string(), "2".to_string()), 0);
    }
    #[test]
    fn test_compare_inf() {
        assert_eq!(compare("3".to_string(), "2".to_string()), -1);
    }
    #[test]
    fn test_compare_sup() {
        assert_eq!(compare("2".to_string(), "3".to_string()), 1);
    }
    #[test]
    fn test_display_equal() {
        assert_eq!(display(1), "<".to_string());
    }
    #[test]
    fn test_display_inf() {
        assert_eq!(display(0), "=".to_string());
    }
    #[test]
    fn test_display_sup() {
        assert_eq!(display(-1), ">".to_string());
    }
}
