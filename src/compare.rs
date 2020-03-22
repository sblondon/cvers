use std::cmp::Ordering;


#[derive(Eq)]
struct Version {
    epoch: u8,
    main: Vec<u32>,
    pre_release: String,
    pre_release_number: u8,
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

        let pre_release_number_order: Option<Ordering> = self.cmp_pre_release_number(other);
        match pre_release_number_order {
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

    fn cmp_pre_release_number(&self, other: &Version) -> Option<Ordering> {
        let order: Ordering = self.pre_release_number.cmp(&other.pre_release_number);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }

    fn cmp_pre_release(&self, other: &Version) -> Option<Ordering> {
        if self.pre_release.len() == 0 && other.pre_release.len() == 0 {
            return None
        } else if self.pre_release.len() == 0 && other.pre_release.len() > 0 {
            return Some(Ordering::Greater)
        } else if self.pre_release.len() > 0 && other.pre_release.len() == 0 {
            return Some(Ordering::Less)
        }

        let order: Ordering = self.pre_release.cmp(&other.pre_release);
        if order == Ordering::Equal {
           return None
        } else {
           return Some(order)
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.main == other.main && self.pre_release_number == other.pre_release_number
    }
}

pub fn compare(raw_version_a: &str, raw_version_b: &str)-> Ordering{
    let mut version_a: Version = parse_raw_version(raw_version_a);
    let mut version_b: Version = parse_raw_version(raw_version_b);

    normalize_length(&mut version_a.main, &mut version_b.main);

    version_a.cmp(&version_b)
}

fn parse_raw_version(raw_version: &str) -> Version{
    let version_without_epoch: String;
    let mut main_version_numbers: Vec<u32> = Vec::new();
    let mut pre_release: String = "".to_string();
    let mut epoch: u8 = 0;
    let mut pre_release_number: u8 = 0;

    let splitted_version: Vec<_> = raw_version.split(':').collect();
    if splitted_version.len() == 2 {
        epoch = splitted_version[0].parse().unwrap();
        version_without_epoch = splitted_version[1].to_string();
    } else {
        version_without_epoch = raw_version.to_string();
    }

    let version_and_prerelease: Vec<_> = version_without_epoch.split('-').collect();
    for element in version_and_prerelease[0].split('.'){
        main_version_numbers.push(element.parse().unwrap());
    }
    if version_and_prerelease.len() == 2{
       if version_and_prerelease[1][..2] == "rc".to_string(){
            pre_release = "rc".to_string();
            pre_release_number = version_and_prerelease[1][2..].parse().unwrap();
        } else {
            if version_and_prerelease[1].find(".") != None {
                let splitted_prerelease: Vec<_> = version_and_prerelease[1].split('.').collect();
                pre_release = splitted_prerelease[0].parse().unwrap();
                pre_release_number = splitted_prerelease[1].parse().unwrap();
            }else{
                pre_release = version_and_prerelease[1].to_string();
            }
        }
    }

    return Version{
        epoch: epoch,
        main: main_version_numbers,
        pre_release: pre_release,
        pre_release_number: pre_release_number,
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
        const VERSION: &str = "3.14159265";
        assert_equal(VERSION, VERSION);
    }
    fn assert_equal(first: &str, second: &str){
        assert_eq!(compare(first, second), Ordering::Equal);
        assert_eq!(compare(second, first), Ordering::Equal);
    }
    #[test]
    fn test_compare_equal() {
        const VERSION: &str = "2";
        assert_equal(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_two_dots() {
        const VERSION: &str = "2.0";
        assert_equal(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_different_dots_quantity() {
        const FIRST: &str = "2.0";
        const SECOND: &str = "2.0";
        assert_equal(FIRST, SECOND);
    }
    #[test]
    fn test_compare_equal_with_alpha() {
        const VERSION: &str = "1.0-alpha";
        assert_equal(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_beta() {
        const VERSION: &str = "1.0-beta";
        assert_equal(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_rc_numbers() {
        // like linux release versions
        const VERSION: &str = "5.5-rc7";
        assert_equal(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_debian_epoch() {
        const VERSION: &str = "1:1.2.3";
        assert_equal(VERSION, VERSION);
    }
    #[test]
    fn test_not_equal_basic() {
        const MAX: &str = "3";
        const MIN: &str = "2";
        assert_not_equal(MAX, MIN);
    }
    fn assert_not_equal(max: &str, min: &str){
        assert_eq!(compare(max, min), Ordering::Greater);
        assert_eq!(compare(min, max), Ordering::Less);
    }
    #[test]
    fn test_not_equal_between_rc_version_and_release_version() {
        // like linux release versions
        const MAX: &str = "5.5";
        const MIN: &str = "5.5-rc6";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_with_two_dots() {
        const MAX: &str = "2.1";
        const MIN: &str = "2.0";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_with_rc_numbers() {
        // like linux release versions
        const MAX: &str = "5.5-rc7";
        const MIN: &str = "5.5-rc6";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_alpha_and_beta_versions() {
        const MAX: &str = "5.5-beta";
        const MIN: &str = "5.5-alpha";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_alpha_sub_versions() {
        const MAX: &str = "5.5-alpha.10";
        const MIN: &str = "5.5-alpha.2";
        assert_not_equal(MAX, MIN);
    }

    #[test]
    fn test_not_equal_between_beta_and_rc_versions() {
        const MAX: &str = "1.0-rc1";
        const MIN: &str = "1.0-beta";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_debian_epoch() {
        const MAX: &str = "2:2";
        const MIN: &str = "1:10";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_epoch_and_no_epoch() {
        const MAX: &str = "1:0.1";
        const MIN: &str = "1.2";
        assert_not_equal(MAX, MIN);
    }

}
