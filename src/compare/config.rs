use super::structs::ParserConfig;

pub fn permissive_parser_config() -> ParserConfig {
    return super::structs::ParserConfig {
        epoch_delimiter: Some(':'),
        pre_release_touchs_digit: None
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_permissive_parser_config() {
        assert_eq!(
            permissive_parser_config(),
            permissive_parser_config()
        );
    }

    #[test]
    fn test_epoch_delimiter_not_equal() {
        let actual = permissive_parser_config();
        let mut expected = permissive_parser_config();
        expected.epoch_delimiter = Some('_');

        assert_ne!(actual, expected);
    }

    #[test]
    fn test_pre_release_touchs_digits_not_equal() {
        let actual = permissive_parser_config();
        let mut expected = permissive_parser_config();
        expected.pre_release_touchs_digit = Some(true);

        assert_ne!(actual, expected);
    }
}
