use std::cmp::Ordering;


pub fn compare(raw_version_a: &str, raw_version_b: &str)-> Ordering{
    let version_a: super::structs::Version = super::parse::parse_raw_version(raw_version_a);
    let version_b: super::structs::Version = super::parse::parse_raw_version(raw_version_b);

    version_a.cmp(&version_b)
}


#[cfg(test)]
mod tests {
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
