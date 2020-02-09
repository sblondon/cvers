use std::fmt::Debug;


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    INF,
    EQU,
    SUP
}

pub fn compare(version_a: String, version_b: String)-> Comparison{
    let mut v_a: Vec<u8> = init_version_numbers(version_a);
    let mut v_b: Vec<u8> = init_version_numbers(version_b);

    normalize_length(&mut v_a, &mut v_b);

    if v_a > v_b {
        return Comparison::SUP;
    } else if v_a == v_b {
        return Comparison::EQU;
    } else {
        return Comparison::INF;
    }

}

fn init_version_numbers(version: String) -> Vec<u8>{
    let mut v: Vec<u8> = Vec::new();
    for element in version.split('.'){
        v.push(element.parse().unwrap());
    }
    return v;
}

fn normalize_length(mut _v_a: &mut Vec<u8>, mut _v_b: &mut Vec<u8>){
    let difference: i32 = _v_a.len() as i32 - _v_b.len() as i32;
    if difference > 0 {
        fill_lacking_numbers(&mut _v_b, difference);
    } else {
        fill_lacking_numbers(&mut _v_a, difference * -1);
    }
}

fn fill_lacking_numbers(v: &mut Vec<u8>, mut size: i32){
        while size != 0 {
            v.push(0);
            size -= 1
        }
}

pub fn display(comparison: Comparison)-> String{
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
    fn test_compare_equal_with_two_dots() {
        assert_eq!(compare("2.0".to_string(), "2.0".to_string()), Comparison::EQU);
    }
    #[test]
    fn test_compare_equal_with_more_dots_in_first_arg() {
        assert_eq!(compare("2.0.0".to_string(), "2".to_string()), Comparison::EQU);
    }
    #[test]
    fn test_compare_equal_with_more_dots_in_second_arg() {
        assert_eq!(compare("2".to_string(), "2.0.0".to_string()), Comparison::EQU);
    }
    #[test]
    fn test_compare_inf() {
        assert_eq!(compare("3".to_string(), "2".to_string()), Comparison::SUP);
    }
    #[test]
    fn test_compare_inf_with_two_dot() {
        assert_eq!(compare("2.0".to_string(), "2.1".to_string()), Comparison::INF);
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
