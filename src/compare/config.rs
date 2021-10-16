use super::structs::ParserConfig;

pub fn default_parser_config() -> ParserConfig {
    return super::structs::ParserConfig {
        epoch_delimiter: Some(':'),
        pre_release_touchs_digit: None
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_default_parser_config() {
        assert_eq!(
            default_parser_config(),
            default_parser_config()
        );
    }

    #[test]
    fn test_epoch_delimiter_not_equal() {
        let actual = default_parser_config();
        let mut expected = default_parser_config();
        expected.epoch_delimiter = Some('_');

        assert_ne!(actual, expected);
    }

    #[test]
    fn test_pre_release_touchs_digits_not_equal() {
        let actual = default_parser_config();
        let mut expected = default_parser_config();
        expected.pre_release_touchs_digit = Some(true);

        assert_ne!(actual, expected);
    }
}
