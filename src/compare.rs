use std::cmp::Ordering;


#[derive(Eq)]
struct Version {
    main: Vec<u32>,
    dev_step: String,
    rc: u8,
}


impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        let main_order: Option<Ordering> = self.cmp_main(other);
        match main_order {
            Some(x) => return x,
            None => {}
        }

        if self.is_rc() != other.is_rc() {
            return self.cmp_is_rc(other)
        } else {
            if self.is_rc() {
                return self.cmp_rc(other)
            } else {
                return self.cmp_dev_step(other)
            }
        }
    }
}

impl Version {
    fn cmp_main(&self, other: &Version) -> Option<Ordering> {
        let order: Ordering = self.main.cmp(&other.main);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }

    fn is_rc(&self) -> bool {
        return self.dev_step == "rc".to_string()
    }
    fn cmp_is_rc(&self, other: &Version) -> Ordering {
        if self.is_rc() {
            return Ordering::Less
        } else {
            return Ordering::Greater
        }
    }

    fn cmp_rc(&self, other: &Version) -> Ordering {
        return self.rc.cmp(&other.rc)
    }

    fn cmp_dev_step(&self, other: &Version) -> Ordering {
        return self.dev_step.cmp(&other.dev_step)
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.main == other.main && self.rc == other.rc
    }
}

pub fn compare(raw_version_a: String, raw_version_b: String)-> Ordering{
    let mut version_a: Version = init_version_numbers(raw_version_a);
    let mut version_b: Version = init_version_numbers(raw_version_b);

    normalize_length(&mut version_a.main, &mut version_b.main);

    version_a.cmp(&version_b)
}

fn init_version_numbers(version: String) -> Version{
    let mut version_numbers_only: Vec<u32> = Vec::new();
    let mut version_and_rc: Vec<String> = Vec::new();
    let mut dev_step: String = "".to_string();
    let mut rc: u8 = 0;
    for element in version.split('-'){
        version_and_rc.push(element.to_string());
    }
    if version_and_rc.len() == 2{
        for element in version_and_rc[0].split('.'){
            version_numbers_only.push(element.parse().unwrap());
        }
        if version_and_rc[1][..2] == "rc".to_string(){
            dev_step = "rc".to_string();
            rc = version_and_rc[1][2..].parse().unwrap();
        } else {
            dev_step = version_and_rc[1].to_string();
        }
    } else{
        for element in version.split('.'){
            version_numbers_only.push(element.parse().unwrap());
        }
    }
    return Version{
        main: version_numbers_only,
        dev_step: dev_step,
        rc: rc,
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    fn test_compare_compatible_with_tex_version() {
        assert_eq!(compare("3.14159265".to_string(), "3.14159265".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal() {
        assert_eq!(compare("2".to_string(), "2".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_two_dots() {
        assert_eq!(compare("2.0".to_string(), "2.0".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_more_dots_in_first_arg() {
        assert_eq!(compare("2.0.0".to_string(), "2".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_more_dots_in_second_arg() {
        assert_eq!(compare("2".to_string(), "2.0.0".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_alpha() {
        // like linux release versions
        assert_eq!(compare("1.0-alpha".to_string(), "1.0-alpha".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_beta() {
        assert_eq!(compare("1.0-beta".to_string(), "1.0-beta".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_rc_numbers() {
        // like linux release versions
        assert_eq!(compare("5.5-rc7".to_string(), "5.5-rc7".to_string()), Ordering::Equal);
    }
    #[test]
    fn test_compare_sup() {
        assert_eq!(compare("3".to_string(), "2".to_string()), Ordering::Greater);
    }
    #[test]
    fn test_compare_sup_between_rc_version_and_release_version() {
        // like linux release versions
        assert_eq!(compare("5.5".to_string(), "5.5-rc6".to_string()), Ordering::Greater);
    }
    #[test]
    fn test_compare_inf_with_two_dots() {
        assert_eq!(compare("2.0".to_string(), "2.1".to_string()), Ordering::Less);
    }
    #[test]
    fn test_compare_inf() {
        assert_eq!(compare("2".to_string(), "3".to_string()), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_with_rc_numbers() {
        // like linux release versions
        assert_eq!(compare("5.5-rc6".to_string(), "5.5-rc7".to_string()), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_between_rc_version_and_release_version() {
        // like linux release versions
        assert_eq!(compare("5.5-rc6".to_string(), "5.5".to_string()), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_between_alpha_and_beta_versions() {
        assert_eq!(compare("5.5-alpha".to_string(), "5.5-beta".to_string()), Ordering::Less);
    }

}
