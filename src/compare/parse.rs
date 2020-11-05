use super::structs::{Version, MainBlock, PrereleaseBlock, BuildBlock};

pub fn parse_raw_version(raw_version: &str) -> Version{
    let version_without_epoch: &str;
    let mut epoch: Option<u8> = None;
    let splitted_epoch_and_tail: Vec<&str> = split_str(raw_version, ':');
    if splitted_epoch_and_tail.len() == 2 {
        epoch = Some(splitted_epoch_and_tail[0].parse().unwrap());
        version_without_epoch = splitted_epoch_and_tail[1];
    } else {
        version_without_epoch = splitted_epoch_and_tail[0];
    }

    let mut prerelease_block: Option<PrereleaseBlock> = None;
    let mut build_block: Option<BuildBlock> = None;
    let version_and_prerelease: Vec<&str> = split_str(&version_without_epoch, '-');
    let version_and_build: Vec<&str> = split_str(&version_without_epoch, '+');
    let main_block: MainBlock;
    if version_and_prerelease.len() == 2 {
        main_block = parse_main_block(version_and_prerelease[0]);
        let raw_prerelease: &str = version_and_prerelease[1];
        prerelease_block = Some(parse_prerelease(&raw_prerelease));
    } else if version_and_build.len() == 2 {
        main_block = parse_main_block(version_and_build[0]);
        let raw_build: &str = version_and_build[1];
        build_block = Some(parse_build(&raw_build));
    } else {
        main_block = parse_main_block(version_and_prerelease[0]);
    }
    Version {
        epoch: epoch,
        main: main_block,
        pre_release: prerelease_block,
        build: build_block,
    }
}

fn split_str(s: &str, delimiter: char) -> Vec<&str> {
    s.split(delimiter).collect()
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
    let splitted_prerelease: Vec<&str> = split_str(raw_prerelease, '.');
    if splitted_prerelease.len() == 2 {
        step = splitted_prerelease[0].parse().unwrap();
        post_number = Some(splitted_prerelease[1].parse().unwrap());
    } else if raw_prerelease[..2] == "rc".to_string() {
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
