use super::structs::{Version, MainBlock, PrereleaseBlock, BuildBlock};

pub fn parse_raw_version(raw_version: &str) -> Version{
    let version_without_epoch: &str;
    let mut epoch: Option<u8> = None;
    let (raw_epoch, tail): (&str, &str) = split_str(raw_version, ':');
    if tail.len() > 0 {
        epoch = Some(raw_epoch.parse().unwrap());
        version_without_epoch = tail;
    } else {
        version_without_epoch = raw_epoch;
    }

    let mut prerelease_block: Option<PrereleaseBlock> = None;
    let mut build_block: Option<BuildBlock> = None;
    let (raw_version, raw_prerelease, raw_build): (&str, &str, &str) = split_version_prerelease_build_str_parts(&version_without_epoch);
    let main_block: MainBlock = parse_main_block(raw_version);
    if raw_prerelease.len() > 0 {
        prerelease_block = Some(parse_prerelease(&raw_prerelease));
    }
    if raw_build.len() > 0 {
        build_block = Some(parse_build(&raw_build));
    }
    Version {
        epoch: epoch,
        main: main_block,
        pre_release: prerelease_block,
        build: build_block,
    }
}

fn split_str(s: &str, delimiter: char) -> (&str, &str) {
    let splitted: Vec<&str> = s.split(delimiter).collect();
    if splitted.len() == 2 {
        (splitted[0], splitted[1])
    } else {
        (splitted[0], "")
    }
}

fn split_version_prerelease_build_str_parts(s: &str) -> (&str, &str, &str) {
    let (part_1, part_2): (&str, &str) = split_str(s, '-');
    if part_2.len() == 0 {
        let (subpart_1, subpart_2): (&str, &str) = split_str(part_1, '+');
        (subpart_1, "", subpart_2)
    } else {
        let (subpart_1, subpart_2): (&str, &str) = split_str(part_2, '+');
        (part_1, subpart_1, subpart_2)
    }
}

fn parse_main_block(raw_main_block: &str) -> MainBlock {
    let mut main_version_numbers: Vec<u32> = Vec::new();
    let mut post_main_letter: Option<char> = None;
    for subversion in raw_main_block.split('.'){
        if last_char_is_letter(&subversion) {
            let index_without_last_char: usize = subversion.chars().count() - 1;
            main_version_numbers.push(subversion[0..index_without_last_char].parse().unwrap());
            post_main_letter = subversion.chars().last();
        } else {
            main_version_numbers.push(subversion.parse().unwrap());
        }
    }
    MainBlock {
        numbers: main_version_numbers,
        post_letter: post_main_letter,
    }
}

fn last_char_is_letter(s: &str) -> bool {
    ! s.chars().last().unwrap().is_digit(10)
}

fn parse_prerelease(raw_prerelease: &str) -> PrereleaseBlock {
    let step: String;
    let mut post_number: Option<u8> = None;
    let (raw_step, raw_number): (&str, &str) = split_str(raw_prerelease, '.');
    if raw_number.len() > 0 {
        step = raw_step.parse().unwrap();
        post_number = Some(raw_number.parse().unwrap());
    } else if raw_prerelease.len() > 2 && raw_prerelease[..2] == "rc".to_string() {
        step = "rc".to_string();
        post_number = Some(raw_prerelease[2..].parse().unwrap());
    } else {
       step = raw_prerelease.parse().unwrap();
    }
    PrereleaseBlock {
        step: step,
        post_number: post_number,
    }
}

fn parse_build(raw_build: &str) -> BuildBlock {
    BuildBlock {
        number: raw_build.parse().unwrap(),
    }
}
