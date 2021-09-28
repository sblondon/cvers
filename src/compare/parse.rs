use super::super::errors;
use super::structs::{Version, MainBlock, PrereleaseBlock, BuildBlock, ParserConfig};

pub fn parse_raw_version(raw_version: &str, parser_config: &ParserConfig) -> Version{
    let (raw_epoch, raw_tail): (&str, &str) = split_epoch_tail(raw_version, parser_config);
    let epoch: Option<u8> = parse_epoch(raw_epoch);

    let (raw_main, raw_prerelease, raw_build): (&str, &str, &str) = split_version_prerelease_build(&raw_tail);
    let main_block: MainBlock = parse_main(raw_main, parser_config);
    let prerelease_block: Option<PrereleaseBlock> = parse_prerelease(&raw_prerelease);
    let build_block: Option<BuildBlock> = parse_build(&raw_build);
    Version {
        epoch: epoch,
        main: main_block,
        pre_release: prerelease_block,
        build: build_block,
    }
}

fn parse_epoch(raw_epoch: &str) -> Option<u8> {
    match raw_epoch {
        "" => None,
        s => Some(s.parse().unwrap()),
    }
}

fn split_epoch_tail<'a>(s: &'a str, parser_config: &ParserConfig) -> (&'a str, &'a str) {
    if parser_config.epoch_separator == None {
        return ("", s)
    }
    let separator = parser_config.epoch_separator.unwrap();
    let splitted: Vec<&str> = s.split(separator).collect();
    match splitted.len() {
        1 => ("", splitted[0]),
        2 => (splitted[0], splitted[1]),
        _ => {
            let message = format!(
                "Error: more than one '{separator}' character for epoch",
                separator=separator,
            ).as_str().to_owned();
            errors::exit_on_error(&message);
        },
    }
}

fn split_str(s: &str, delimiter: char) -> (&str, &str) {
    let splitted: Vec<&str> = s.split(delimiter).collect();
    match splitted.len() {
        1 => (splitted[0], ""),
        2 => (splitted[0], splitted[1]),
        _ => {
            let message = format!(
                "Error: more than one '{delimiter}' character",
                delimiter=delimiter.to_string()
            ).as_str().to_owned();
            errors::exit_on_error(&message);
        },
    }
}

fn split_version_prerelease_build(s: &str) -> (&str, &str, &str) {
    let (part_1, part_2): (&str, &str) = split_str(s, '-');
    match part_2.len() {
        0 => {
            let (subpart_1, subpart_2): (&str, &str) = split_str(part_1, '+');
            (subpart_1, "", subpart_2)
        },
        _ => {
            let (subpart_1, subpart_2): (&str, &str) = split_str(part_2, '+');
            (part_1, subpart_1, subpart_2)
        },
    }
}

fn parse_main(raw_main_block: &str, parser_config: &ParserConfig) -> MainBlock {
    let mut main_version_numbers: Vec<u32> = Vec::new();
    let mut pre_main_letter: Option<char> = None;
    let mut post_main_letter: Option<char> = None;
    for subversion in raw_main_block.split('.'){
        if last_char_is_letter(&subversion) {
            if subversion.chars().count() > 1 {
                let index_without_last_char: usize = subversion.chars().count() - 1;
                main_version_numbers.push(subversion[0..index_without_last_char].parse().unwrap());
            }
            let last_char: Option<char> = subversion.chars().last();
            match parser_config.pre_release_touchs_digit {
                Some(true) => {pre_main_letter = last_char;},
                Some(false) => {post_main_letter = last_char;},
                None => {
                    let message = format!(
                        "Error: invalid letter ('{letter}') in '{block}'",
                        letter=last_char.unwrap(),
                        block=raw_main_block.to_string()
                    ).as_str().to_owned();
                    errors::exit_on_error(&message);
                },
            }
        } else {
            main_version_numbers.push(subversion.parse().unwrap());
        }
    }
    MainBlock {
        numbers: main_version_numbers,
        pre_letter: pre_main_letter,
        post_letter: post_main_letter,
    }
}

fn last_char_is_letter(s: &str) -> bool {
    ! s.chars().last().unwrap().is_digit(10)
}

fn parse_prerelease(raw_prerelease: &str) -> Option<PrereleaseBlock> {
    if raw_prerelease == "" {
        return None
    }

    let step: String;
    let mut post_number: Option<u8> = None;
    let mut post_step: Option<String> = None;
    let (raw_step, raw_second_elem): (&str, &str) = split_str(raw_prerelease, '.');
    if raw_second_elem.len() > 0 {
        step = raw_step.parse().unwrap();
        if raw_second_elem.chars().nth(0).unwrap().is_digit(10) {
            post_number = Some(raw_second_elem.parse().unwrap());
        } else {
            post_step = Some(raw_second_elem.parse().unwrap());
        }
    } else if raw_prerelease.len() > 2 && raw_prerelease[..2].to_lowercase() == "rc".to_string() {
        step = "rc".to_string();
        post_number = Some(raw_prerelease[2..].parse().unwrap());
    } else {
       step = raw_prerelease.parse().unwrap();
    }
    Some(PrereleaseBlock {
        step: step,
        post_number: post_number,
        post_step: post_step,
    })
}

fn parse_build(raw_build: &str) -> Option<BuildBlock> {
    match raw_build {
        "" => None,
        s => Some(BuildBlock {
            number: s.parse().unwrap(),
        })
    }
}
