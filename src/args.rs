use super::compare::{permissive_parser_config, ParserConfig};

pub fn parse_arguments(args: Vec<String>) -> (ParserConfig, Vec::<String>) {
    let mut parser_config = permissive_parser_config();
    let mut mandatories_args: Vec<String> = Vec::new();
    for arg in args {
        let arg_str = arg.as_str();
        match arg_str {
            "--pre-release-touchs-digit" => {
                parser_config.pre_release_touchs_digit = Some(true);
            },
            _ => mandatories_args.push(arg),
        }
    }
    (parser_config, mandatories_args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mandatory_parameters() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("verb"));
        args.push(String::from("first value"));
        args.push(String::from("second value"));
        let mut expected_args: Vec<String> = Vec::new();
        expected_args.push(String::from("verb"));
        expected_args.push(String::from("first value"));
        expected_args.push(String::from("second value"));

        let parsed_args: (ParserConfig, Vec::<String>) = parse_arguments(args);

        assert_eq!(parsed_args.0, super::super::compare::permissive_parser_config());
        assert_eq!(parsed_args.1, expected_args);
    }

    #[test]
    fn test_default_config() {
        let args: Vec<String> = Vec::new();
        let expected_args: Vec<String> = Vec::new();

        let parsed_args: (ParserConfig, Vec::<String>) = parse_arguments(args);

        assert_eq!(parsed_args.0, super::super::compare::permissive_parser_config());
        assert_eq!(parsed_args.1, expected_args);
    }

    #[test]
    fn test_enable_pre_release_touchs_digit_option() {
        let mut args: Vec<String> = Vec::new();
        args.push(String::from("--pre-release-touchs-digit"));
        let mut expected: ParserConfig = super::super::compare::permissive_parser_config();
        expected.pre_release_touchs_digit = Some(true);

        assert_eq!(parse_arguments(args).0, expected);
    }

}
