use std::fmt::Debug;


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    INF,
    EQU,
    SUP
}

pub fn compare(raw_version_a: String, raw_version_b: String)-> Comparison{
    let mut version_a: Vec<u32> = init_version_numbers(raw_version_a);
    let mut version_b: Vec<u32> = init_version_numbers(raw_version_b);

    normalize_length(&mut version_a, &mut version_b);

    if version_a > version_b {
        return Comparison::SUP;
    } else if version_a == version_b {
        return Comparison::EQU;
    } else {
        return Comparison::INF;
    }

}

fn init_version_numbers(version: String) -> Vec<u32>{
    let mut v: Vec<u32> = Vec::new();
    for element in version.split('.'){
        v.push(element.parse().unwrap());
    }
    return v;
}

fn normalize_length(mut _version_a: &mut Vec<u32>, mut _version_b: &mut Vec<u32>){
    let difference: i32 = _version_a.len() as i32 - _version_b.len() as i32;
    if difference > 0 {
        fill_lacking_numbers(&mut _version_b, difference);
    } else {
        fill_lacking_numbers(&mut _version_a, difference * -1);
    }
}

fn fill_lacking_numbers(fillable: &mut Vec<u32>, mut size: i32){
        while size != 0 {
            fillable.push(0);
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
    fn test_compare_compatible_with_tex_version() {
        assert_eq!(compare("3.14159265".to_string(), "3.14159265".to_string()), Comparison::EQU);
    }
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
