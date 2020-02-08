use std::fmt::Debug;


#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    INF,
    EQU,
    SUP
}

pub fn compare(version_a: String, version_b: String)-> Comparison{
    let mut v_a: Vec<i32> = Vec::new();
    for element in version_a.split('.'){
        v_a.push(element.parse().unwrap());
    }
    let mut v_b: Vec<i32> = Vec::new();
    for element in version_b.split('.'){
        v_b.push(element.parse().unwrap());
    }

    normalize_length(&mut v_a, &mut v_b);

    if v_a > v_b {
        return Comparison::SUP;
    } else if v_a == v_b {
        return Comparison::EQU;
    } else {
        return Comparison::INF;
    }

}

fn normalize_length(_v_a: &mut Vec<i32>, _v_b: &mut Vec<i32>){
    if _v_a.len() > _v_b.len(){
        let mut difference: usize = _v_a.len() - _v_b.len();
        while difference != 0 {
            _v_b.push(0);
            difference -= 1
        }
    } else if _v_a.len() < _v_b.len(){
        let mut difference: usize = _v_b.len() - _v_a.len();
        while difference != 0 {
            _v_a.push(0);
            difference -= 1
        }
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
