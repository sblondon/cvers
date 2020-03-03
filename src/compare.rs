use std::cmp::Ordering;


#[derive(Eq)]
struct Version {
    epoch: u8,
    main: Vec<u32>,
    pre_release: String,
    rc: u8,
}


impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        let epoch_order: Option<Ordering> = self.cmp_epoch(other);
        match epoch_order {
            Some(x) => return x,
            None => {}
        }

        let main_order: Option<Ordering> = self.cmp_main(other);
        match main_order {
            Some(x) => return x,
            None => {}
        }

        let pre_release_order: Option<Ordering> = self.cmp_pre_release(other);
        match pre_release_order {
            Some(x) => return x,
            None => {}
        }

        let rc_order: Option<Ordering> = self.cmp_rc(other);
        match rc_order {
            Some(x) => return x,
            None => {}
        }

        return Ordering::Equal
    }
}

impl Version {
    fn cmp_epoch(&self, other: &Version) -> Option<Ordering> {
        let order: Ordering = self.epoch.cmp(&other.epoch);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }

   fn cmp_main(&self, other: &Version) -> Option<Ordering> {
        let order: Ordering = self.main.cmp(&other.main);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }

    fn is_rc(&self) -> bool {
        return self.pre_release == "rc".to_string()
    }

    fn cmp_rc(&self, other: &Version) -> Option<Ordering> {
        if self.is_rc() {
            return Some(self.rc.cmp(&other.rc))
        } else {
            return None
        }
    }

    fn cmp_pre_release(&self, other: &Version) -> Option<Ordering> {
        if self.pre_release.len() == 0 && other.pre_release.len() == 0 {
            return None
        }
        if self.pre_release.len() > 0 && other.pre_release.len() > 0 {
            if self.is_rc() && other.is_rc(){
                return None
            } else {
                return Some(self.pre_release.cmp(&other.pre_release))
            }
        }
        if self.pre_release.len() == 0 {
            return Some(Ordering::Greater)
        } else {
            return Some(Ordering::Less)
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.main == other.main && self.rc == other.rc
    }
}

pub fn compare(raw_version_a: &str, raw_version_b: &str)-> Ordering{
    let mut version_a: Version = init_version_numbers(raw_version_a.to_string());
    let mut version_b: Version = init_version_numbers(raw_version_b.to_string());

    normalize_length(&mut version_a.main, &mut version_b.main);

    version_a.cmp(&version_b)
}

fn init_version_numbers(raw_version: String) -> Version{
    let version_without_epoch: String;
    let mut version_numbers_only: Vec<u32> = Vec::new();
    let mut version_and_rc: Vec<String> = Vec::new();
    let mut pre_release: String = "".to_string();
    let mut epoch: u8 = 0;
    let mut rc: u8 = 0;
    if raw_version.find(':') != None {
        let splitted_version: Vec<_> = raw_version.split(':').collect();
        epoch = splitted_version[0].parse().unwrap();
        version_without_epoch = splitted_version[1].to_string();
    } else {
        version_without_epoch = raw_version;
    }
    for element in version_without_epoch.split('-'){
        version_and_rc.push(element.to_string());
    }
    if version_and_rc.len() == 2{
        for element in version_and_rc[0].split('.'){
            version_numbers_only.push(element.parse().unwrap());
        }
        if version_and_rc[1][..2] == "rc".to_string(){
            pre_release = "rc".to_string();
            rc = version_and_rc[1][2..].parse().unwrap();
        } else {
            pre_release = version_and_rc[1].to_string();
        }
    } else{
        for element in version_without_epoch.split('.'){
            version_numbers_only.push(element.parse().unwrap());
        }
    }
    return Version{
        epoch: epoch,
        main: version_numbers_only,
        pre_release: pre_release,
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
        assert_eq!(compare(&"3.14159265", &"3.14159265"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal() {
        assert_eq!(compare(&"2", &"2"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_two_dots() {
        assert_eq!(compare(&"2.0", &"2.0"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_more_dots_in_first_arg() {
        assert_eq!(compare(&"2.0.0", &"2"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_more_dots_in_second_arg() {
        assert_eq!(compare(&"2", &"2.0.0"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_alpha() {
        assert_eq!(compare(&"1.0-alpha", &"1.0-alpha"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_beta() {
        assert_eq!(compare(&"1.0-beta", &"1.0-beta"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_rc_numbers() {
        // like linux release versions
        assert_eq!(compare(&"5.5-rc7", &"5.5-rc7"), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal_with_debian_epoch() {
        assert_eq!(compare(&"1:1.2.3", &"1:1.2.3"), Ordering::Equal);
    }
    #[test]
    fn test_compare_sup() {
        assert_eq!(compare(&"3", &"2"), Ordering::Greater);
    }
    #[test]
    fn test_compare_sup_between_rc_version_and_release_version() {
        // like linux release versions
        assert_eq!(compare(&"5.5", &"5.5-rc6"), Ordering::Greater);
    }
    #[test]
    fn test_compare_inf_with_two_dots() {
        assert_eq!(compare(&"2.0", &"2.1"), Ordering::Less);
    }
    #[test]
    fn test_compare_inf() {
        assert_eq!(compare(&"2", &"3"), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_with_rc_numbers() {
        // like linux release versions
        assert_eq!(compare(&"5.5-rc6", &"5.5-rc7"), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_between_rc_version_and_release_version() {
        // like linux release versions
        assert_eq!(compare(&"5.5-rc6", &"5.5"), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_between_alpha_and_beta_versions() {
        assert_eq!(compare(&"5.5-alpha", &"5.5-beta"), Ordering::Less);
    }
    #[test]
    fn test_compare_inf_between_beta_and_rc_versions() {
        assert_eq!(compare(&"1.0-beta", &"1.0-rc1"), Ordering::Less);
    }
    #[test]
    fn test_compare_between_debian_epoch() {
        assert_eq!(compare(&"1:10", &"2:2"), Ordering::Less);
    }
    #[test]
    fn test_compare_between_epoch_and_no_epoch() {
        assert_eq!(compare(&"1.2", &"1:0.1"), Ordering::Less);
    }

}
