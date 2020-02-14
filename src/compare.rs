use std::fmt::Debug;
use std::cmp::Ordering;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Comparison {
    INF,
    EQU,
    SUP
}

#[derive(Eq)]
struct Version {
    main: Vec<u32>,
}


impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        self.main.cmp(&other.main)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.main == other.main
    }
}

pub fn compare(raw_version_a: String, raw_version_b: String)-> Comparison{
    let mut version_a: Version = init_version_numbers(raw_version_a);
    let mut version_b: Version = init_version_numbers(raw_version_b);

    normalize_length(&mut version_a.main, &mut version_b.main);

    if version_a > version_b {
        return Comparison::SUP;
    } else if version_a.main == version_b.main {
        return Comparison::EQU;
    } else {
        return Comparison::INF;
    }

}

fn init_version_numbers(version: String) -> Version{
    let mut version_numbers_only: Vec<u32> = Vec::new();
    let mut version_and_rc: Vec<String> = Vec::new();
    for element in version.split('-'){
        version_and_rc.push(element.to_string());
    }
    if version_and_rc.len() == 2{
        for element in version_and_rc[0].split('.'){
            version_numbers_only.push(element.parse().unwrap());
        }
    } else{
        for element in version.split('.'){
            version_numbers_only.push(element.parse().unwrap());
        }
    }
    return Version{
        main: version_numbers_only
    }
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
    fn test_compare_equal_with_rc_numbers() {
        // like linux release versions
        assert_eq!(compare("5.5-rc7".to_string(), "5.5-rc7".to_string()), Comparison::EQU);
    }
    #[test]
    fn test_compare_sup() {
        assert_eq!(compare("3".to_string(), "2".to_string()), Comparison::SUP);
    }
    #[test]
    fn test_compare_inf_with_two_dots() {
        assert_eq!(compare("2.0".to_string(), "2.1".to_string()), Comparison::INF);
    }
    #[test]
    fn test_compare_inf() {
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
