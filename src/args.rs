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
    fn test_default_config() {
        let args: Vec<String> = Vec::new();
        let expected_args: Vec<String> = Vec::new();
        let expected = (
            super::super::compare::permissive_parser_config(),
            expected_args
        );
        assert_eq!(parse_arguments(args), expected);
    }

}
