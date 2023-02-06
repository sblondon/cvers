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
        let args: Vec<String> = vec![
            String::from("verb"),
            String::from("first value"),
            String::from("second value")
        ];
        let expected_args: Vec<String> = vec![
            String::from("verb"),
            String::from("first value"),
            String::from("second value")
        ];

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
        let args: Vec<String> = vec![
            String::from("--pre-release-touchs-digit")
        ];
        let mut expected: ParserConfig = super::super::compare::permissive_parser_config();
        expected.pre_release_touchs_digit = Some(true);

        assert_eq!(parse_arguments(args).0, expected);
    }

}
