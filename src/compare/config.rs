use super::structs::ParserConfig;

pub fn default_parser_config() -> ParserConfig {
    return super::structs::ParserConfig {
        epoch_delimiter: Some(':'),
        pre_release_touchs_digit: None
    };
}