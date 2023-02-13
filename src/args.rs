use super::compare::{permissive_parser_config, ParserConfig};

pub fn parse_arguments(args: Vec<String>) -> (ParserConfig, Vec::<String>) {
    let mut parser_config = permissive_parser_config();
    let mut mandatories_args: Vec<String> = Vec::new();
    let mut next_args_is_epoch_delimiter: bool = false;
    for arg in args {
        let arg_str = arg.as_str();
        match arg_str {
            "--pre-release-touchs-digit" => {
                parser_config.pre_release_touchs_digit = Some(true);
            },
            "--epoch" => {
                next_args_is_epoch_delimiter = true;
            },
            _ => {
                if next_args_is_epoch_delimiter {
                    parser_config.epoch_delimiter = arg.chars().next();
                    next_args_is_epoch_delimiter = false;
                } else {
                    mandatories_args.push(arg)
                }
            },
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

        let parsed_args: (ParserConfig, Vec::<String>) = parse_arguments(args.clone());

        assert_eq!(parsed_args.1, args);
    }

    #[test]
    fn test_default_config() {
        let args: Vec<String> = Vec::new();

        let parsed_args: (ParserConfig, Vec::<String>) = parse_arguments(args);

        assert_eq!(parsed_args.0, super::super::compare::permissive_parser_config());
    }

    #[test]
    fn test_enable_pre_release_touchs_digit_option() {
        let args: Vec<String> = vec![
            String::from("--pre-release-touchs-digit"),
            String::from("verb"),
            String::from("first value"),
            String::from("second value")
        ];
        let mandatory_args: Vec<String> = vec![
            args[1].clone(),
            args[2].clone(),
            args[3].clone(),
        ];

        let mut expected: ParserConfig = super::super::compare::permissive_parser_config();
        expected.pre_release_touchs_digit = Some(true);

        let parsed_args: (ParserConfig, Vec::<String>) = parse_arguments(args);

        assert_eq!(parsed_args.0, expected);
        assert_eq!(parsed_args.1, mandatory_args);
    }

    #[test]
    fn test_set_epoch_character() {
        let args: Vec<String> = vec![
            String::from("--epoch"),
            String::from("|"),
            String::from("verb"),
            String::from("first value"),
            String::from("second value")
        ];
        let mandatory_args: Vec<String> = vec![
            args[2].clone(),
            args[3].clone(),
            args[4].clone(),
        ];

        let mut expected: ParserConfig = super::super::compare::permissive_parser_config();
        expected.epoch_delimiter = Some('|');

        let parsed_args: (ParserConfig, Vec::<String>) = parse_arguments(args);

        assert_eq!(parsed_args.0, expected);
        assert_eq!(parsed_args.1, mandatory_args);
    }

}
