use std::cmp::Ordering;


pub fn compare_with_operator(raw_version_a: &str, raw_version_b: &str, raw_operator: &str, parser_config: &super::structs::ParserConfig)-> bool{
    let order = compare(raw_version_a, raw_version_b, &parser_config);

    (order == Ordering::Less && (raw_operator == "<<" || raw_operator == "<=")) ||
        (order == Ordering::Greater && (raw_operator == ">>" || raw_operator == "=>")) ||
        (order == Ordering::Equal && (raw_operator == "<=" || raw_operator == "==" || raw_operator == "=>"))
}

pub fn compare(raw_version_a: &str, raw_version_b: &str, parser_config: &super::structs::ParserConfig)-> Ordering{
    let version_a: super::structs::Version = super::parse::parse_raw_version(raw_version_a, parser_config);
    let version_b: super::structs::Version = super::parse::parse_raw_version(raw_version_b, parser_config);

    version_a.cmp(&version_b)
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::structs;

    #[test]
    fn test_compare_compatible_with_tex_version() {
        const VERSION: &str = "3.14159265";
        assert_equal_with_default_parser(VERSION, VERSION);
    }

    fn assert_equal_with_default_parser(max: &str, min: &str){
        let parser_config: structs::ParserConfig = default_parser_config();

        assert_equal(max, min, &parser_config);
    }

    fn assert_equal(first: &str, second: &str, parser_config: &structs::ParserConfig){
        assert_eq!(compare(first, second, &parser_config), Ordering::Equal);
        assert_eq!(compare(second, first, &parser_config), Ordering::Equal);
    }

    fn default_parser_config() -> structs::ParserConfig {
        return structs::ParserConfig {
            epoch_separator: Some(':'),
            pre_release_touchs_digit: None
        };
    }

    fn python_parser_config() -> structs::ParserConfig {
        return structs::ParserConfig {
            epoch_separator: Some('!'),
            pre_release_touchs_digit: Some(true),
        };
    }

    fn openssl_parser_config() -> structs::ParserConfig {
        return structs::ParserConfig {
            epoch_separator: None,
            pre_release_touchs_digit: Some(false)
        };
    }

    #[test]
    fn test_compare_equal() {
        const VERSION: &str = "2";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_two_dots() {
        const VERSION: &str = "2.0.0";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_different_dots_quantity() {
        const FIRST: &str = "2";
        const SECOND: &str = "2.0.0";
        assert_equal_with_default_parser(FIRST, SECOND);
        assert_equal_with_default_parser(SECOND, FIRST);
    }
    #[test]
    fn test_compare_equal_with_alpha() {
        const VERSION: &str = "1.0-alpha";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_beta() {
        const VERSION: &str = "1.0-beta";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_rc_numbers() {
        // like linux release versions
        const VERSION: &str = "5.5-rc7";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_insensitive_case_rc_numbers() {
        // https://www.python.org/dev/peps/pep-0440/#case-sensitivity
        const VERSION_LOWERCASE: &str = "1.1-rc1";
        const VERSION_UPPERCASE: &str = "1.1-RC1";
        assert_equal_with_default_parser(VERSION_LOWERCASE, VERSION_UPPERCASE);
    }
    #[test]
    fn test_compare_equal_with_build_number() {
        const VERSION: &str = "1.0+1";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_debian_epoch() {
        const VERSION: &str = "1:1.2.3";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_compare_equal_with_python_epoch() {
        // https://www.python.org/dev/peps/pep-0440/
        const VERSION: &str = "1!1.2.3";
        let parser_config: structs::ParserConfig = python_parser_config();

        assert_equal(VERSION, VERSION, &parser_config);
    }
    #[test]
    fn test_equal_between_alpha_and_beta_sub_version() {
        const VERSION: &str = "1.0.0-alpha.beta";
        assert_equal_with_default_parser(VERSION, VERSION);
    }
    #[test]
    fn test_not_equal_basic() {
        const MAX: &str = "3";
        const MIN: &str = "2";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    fn assert_not_equal_with_default_parser(max: &str, min: &str){
        let parser_config: structs::ParserConfig = default_parser_config();

        assert_not_equal(max, min, &parser_config);
    }
    fn assert_not_equal(max: &str, min: &str, parser_config: &structs::ParserConfig){

        assert_eq!(compare(max, min, parser_config), Ordering::Greater);
        assert_eq!(compare(min, max, parser_config), Ordering::Less);
    }
    #[test]
    fn test_not_equal_between_rc_version_and_release_version() {
        // like linux release versions
        const MAX: &str = "5.5";
        const MIN: &str = "5.5-rc6";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_with_two_dots() {
        const MAX: &str = "2.1";
        const MIN: &str = "2.0";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_with_rc_numbers_as_linux_versionning_scheme() {
        const MAX: &str = "5.5-rc7";
        const MIN: &str = "5.5-rc6";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_with_rc_numbers_as_linux_versionning_scheme_with_10_value() {
        const MAX: &str = "3.1-rc10";
        const MIN: &str = "3.1-rc9";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_build_number() {
        const MAX: &str = "1.0+3";
        const MIN: &str = "1.0+1";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_with_rc_numbers_between_build_number() {
        const MAX: &str = "1.0-rc1+3";
        const MIN: &str = "1.0-rc1+1";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_build_number_and_no_build() {
        const MAX: &str = "1.0+3";
        const MIN: &str = "1.0";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_alpha_and_beta_versions() {
        const MAX: &str = "5.5-beta";
        const MIN: &str = "5.5-alpha";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_alpha_sub_versions() {
        const MAX: &str = "5.5-alpha.10";
        const MIN: &str = "5.5-alpha.2";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_rc_sub_versions() {
        const MAX: &str = "5.5-rc.10";
        const MIN: &str = "5.5-rc.2";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_alpha_and_beta_sub_version() {
        const MAX: &str = "1.0.0-alpha.beta";
        const MIN: &str = "1.0.0-alpha.1";
        assert_not_equal_with_default_parser(MAX, MIN);
     }
    #[test]
    fn test_not_equal_between_minor_number_followed_by_letter_considered_postrelease() {
        // like openssl versions
        const MAX: &str = "1.0.2e";
        const MIN: &str = "1.0.2d";
        let parser_config: structs::ParserConfig = openssl_parser_config();

        assert_not_equal(MAX, MIN, &parser_config);
    }
    #[test]
    fn test_not_equal_between_minor_number_followed_by_letter_and_no_letter_considered_postrelease() {
        // like openssl versions
        const MAX: &str = "1.0.2a";
        const MIN: &str = "1.0.2";
        let parser_config: structs::ParserConfig = openssl_parser_config();

        assert_not_equal(MAX, MIN, &parser_config);
    }
    #[test]
    fn test_not_equal_between_minor_number_followed_by_letter_considered_prerelease() {
        // like python PEP: https://www.python.org/dev/peps/pep-0440
        const MAX: &str = "1.0.2e";
        const MIN: &str = "1.0.2d";
        let parser_config: structs::ParserConfig = python_parser_config();

        assert_not_equal(MAX, MIN, &parser_config);
    }
    #[test]
    fn test_not_equal_between_minor_number_followed_by_letter_and_no_letter_considered_prerelease() {
        // like python PEP: https://www.python.org/dev/peps/pep-0440
        const MAX: &str = "1.0.2";
        const MIN: &str = "1.0.2a";
        let parser_config: structs::ParserConfig = python_parser_config();

        assert_not_equal(MAX, MIN, &parser_config);
    }
    #[test]
    fn test_not_equal_between_minor_letter() {
        // like raku langage specifications
        const MAX: &str = "6.d";
        const MIN: &str = "6.c";
        let parser_config: structs::ParserConfig = structs::ParserConfig {
            epoch_separator: None,
            pre_release_touchs_digit: Some(true)
        };

        assert_not_equal(MAX, MIN, &parser_config);
    }

    #[test]
    fn test_not_equal_between_beta_and_rc_versions() {
        const MAX: &str = "1.0-rc1";
        const MIN: &str = "1.0-beta";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_alpha_and_released_versions() {
        const MAX: &str = "5.5";
        const MIN: &str = "5.5-alpha";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_debian_epoch() {
        const MAX: &str = "2:2";
        const MIN: &str = "1:10";
        assert_not_equal_with_default_parser(MAX, MIN);
    }
    #[test]
    fn test_not_equal_between_epoch_and_no_epoch() {
        const MAX: &str = "1:0.1";
        const MIN: &str = "1.2";
        assert_not_equal_with_default_parser(MAX, MIN);
    }

    #[test]
    fn test_match_operator_for_different_versions() {
        let parser_config: structs::ParserConfig = default_parser_config();
        const MAX: &str = "2";
        const MIN: &str = "1";

        assert!(compare_with_operator(MIN, MAX, "<<", &parser_config));
        assert!(compare_with_operator(MIN, MAX, "<=", &parser_config));
        assert_eq!(compare_with_operator(MIN, MAX, "==", &parser_config), false);
        assert!(compare_with_operator(MAX, MIN, "=>", &parser_config));
        assert!(compare_with_operator(MAX, MIN, ">>", &parser_config));
        assert_eq!(compare_with_operator(MAX, MIN, "<<", &parser_config), false);
        assert_eq!(compare_with_operator(MAX, MIN, "<=", &parser_config), false);
        assert_eq!(compare_with_operator(MIN, MAX, "=>", &parser_config), false);
        assert_eq!(compare_with_operator(MIN, MAX, ">>", &parser_config), false);
    }
    #[test]
    fn test_match_operator_for_same_version() {
        let parser_config: structs::ParserConfig = default_parser_config();
        const VERSION: &str = "2";

        assert_eq!(compare_with_operator(VERSION, VERSION, "<<", &parser_config), false);
        assert!(compare_with_operator(VERSION, VERSION, "<=", &parser_config));
        assert!(compare_with_operator(VERSION, VERSION, "==", &parser_config));
        assert!(compare_with_operator(VERSION, VERSION, "=>", &parser_config));
        assert_eq!(compare_with_operator(VERSION, VERSION, ">>", &parser_config), false);
    }

}
