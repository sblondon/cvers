use std::cmp::Ordering;


#[derive(Eq)]
struct Version {
    epoch: u8,
    main: MainBlock,
    pre_release: PrereleaseBlock,
}

#[derive(Eq)]
struct MainBlock {
    numbers: Vec<u32>,
    post_letter: Option<char>,
}

#[derive(Eq)]
struct PrereleaseBlock {
    step: String,
    post_number: u8,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for MainBlock {
    fn partial_cmp(&self, other: &MainBlock) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for PrereleaseBlock {
    fn partial_cmp(&self, other: &PrereleaseBlock) -> Option<Ordering> {
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

        let main_order: Ordering = self.main.cmp(&other.main);
        if main_order != Ordering::Equal {
            return main_order
        }

        let pre_release_order: Ordering = self.pre_release.cmp(&other.pre_release);
        if pre_release_order != Ordering::Equal {
            return pre_release_order
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
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.main == other.main && self.pre_release == other.pre_release
    }
}

impl PartialEq for MainBlock {
    fn eq(&self, other: &MainBlock) -> bool {
        self.numbers == other.numbers && self.post_letter == other.post_letter
    }
}

impl PartialEq for PrereleaseBlock {
    fn eq(&self, other: &PrereleaseBlock) -> bool {
        self.step == other.step && self.post_number == other.post_number
    }
}

impl Ord for MainBlock {
    fn cmp(&self, other: &MainBlock) -> Ordering {
        let order: Option<Ordering> = self.cmp_numbers(other);
        match order {
            Some(x) => return x,
            None => {}
        }

        let order: Option<Ordering> = self.cmp_post_letter(other);
        match order {
            Some(x) => return x,
            None => {}
        }
        return Ordering::Equal
    }
}

impl MainBlock {
    fn cmp_numbers(&self, other: &MainBlock) -> Option<Ordering> {
        let order: Ordering = self.numbers.cmp(&other.numbers);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }

    fn cmp_post_letter(&self, other: &MainBlock) -> Option<Ordering> {
        match [self.post_letter, other.post_letter] {
            [None, None] => return None,
            [Some(_), None] => return Some(Ordering::Greater),
            [None, Some(_)] => return Some(Ordering::Less),
            _ => {}
        }

        let order: Ordering = self.post_letter.cmp(&other.post_letter);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }
}


impl Ord for PrereleaseBlock {
    fn cmp(&self, other: &PrereleaseBlock) -> Ordering {
        let order: Option<Ordering> = self.cmp_step(other);
        match order {
            Some(x) => return x,
            None => {}
        }

        let order: Option<Ordering> = self.cmp_post_number(other);
        match order {
            Some(x) => return x,
            None => {}
        }
        return Ordering::Equal
    }
}

impl PrereleaseBlock {
    fn cmp_post_number(&self, other: &PrereleaseBlock) -> Option<Ordering> {
        let order: Ordering = self.post_number.cmp(&other.post_number);
        if order == Ordering::Equal{
            return None
        } else {
            return Some(order)
        }
    }

    fn cmp_step(&self, other: &PrereleaseBlock) -> Option<Ordering> {
        match [self.step.len(), other.step.len()] {
            [0, 0]  => return None,
            [0, x] if x > 0 => return Some(Ordering::Greater),
            [x, 0] if x > 0 => return Some(Ordering::Less),
            _ => {}
        }

        let order: Ordering = self.step.cmp(&other.step);
        if order == Ordering::Equal {
           return None
        } else {
           return Some(order)
        }
    }
}


pub fn compare(raw_version_a: &str, raw_version_b: &str)-> Ordering{
    let version_a: Version = parse_raw_version(raw_version_a);
    let version_b: Version = parse_raw_version(raw_version_b);

    version_a.cmp(&version_b)
}

fn parse_raw_version(raw_version: &str) -> Version{
    let version_without_epoch: String;
    let mut epoch: u8 = 0;
    let splitted_epoch_and_tail: Vec<_> = raw_version.split(':').collect();
    if splitted_epoch_and_tail.len() == 2 {
        epoch = splitted_epoch_and_tail[0].parse().unwrap();
        version_without_epoch = splitted_epoch_and_tail[1].to_string();
    } else {
        version_without_epoch = splitted_epoch_and_tail[0].to_string();
    }

    let version_and_prerelease: Vec<_> = version_without_epoch.split('-').collect();
    let main_block: MainBlock = parse_main_block(version_and_prerelease[0].to_string());
    let prerelease_block: PrereleaseBlock;
    if version_and_prerelease.len() == 2{
        let raw_prerelease: String = version_and_prerelease[1].to_string();
        prerelease_block = parse_prerelease(raw_prerelease);
    } else {
        prerelease_block = PrereleaseBlock {
            step: "".to_string(),
            post_number: 0,
        };
    }
    return Version {
        epoch: epoch,
        main: main_block,
        pre_release: prerelease_block,
    }
}

fn parse_main_block(raw_main_block: String) -> MainBlock {
    let mut main_version_numbers: Vec<u32> = Vec::new();
    let mut post_main_letter: Option<char> = None;
    for element in raw_main_block.split('.'){
        let subversion: String = element.to_string();
        let index_without_last_char: usize = subversion.chars().count() - 1;
        let last_char_is_letter: bool = ! element.chars().last().unwrap().is_digit(10);
        if last_char_is_letter {
            main_version_numbers.push(element[0..index_without_last_char].parse().unwrap());
            post_main_letter = subversion.chars().rev().next();
        } else {
            main_version_numbers.push(element.parse().unwrap());
        }
    }
    return MainBlock {
        numbers: main_version_numbers,
        post_letter: post_main_letter,
    }
}

fn parse_prerelease(raw_prerelease: String) -> PrereleaseBlock {
    let step: String;
    let mut post_number: u8 = 0;
    let splitted_prerelease: Vec<_> = raw_prerelease.split('.').collect();
    if splitted_prerelease.len() == 2 {
        step = splitted_prerelease[0].parse().unwrap();
        post_number = splitted_prerelease[1].parse().unwrap();
    } else if raw_prerelease[..2] == "rc".to_string() {
        step = "rc".to_string();
        post_number = raw_prerelease[2..].parse().unwrap();
    } else {
       step = raw_prerelease.parse().unwrap();
    }
    return PrereleaseBlock {
        step: step,
        post_number: post_number,
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
    fn test_not_equal_between_rc_sub_versions() {
        const MAX: &str = "5.5-rc.10";
        const MIN: &str = "5.5-rc.2";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_minor_number_followed_by_letter() {
        // like openssl versions
        const MAX: &str = "1.0.2e";
        const MIN: &str = "1.0.2d";
        assert_not_equal(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_minor_number_followed_by_letter_and_no_letter() {
        // like openssl versions
        const MAX: &str = "1.0.2a";
        const MIN: &str = "1.0.2";
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
