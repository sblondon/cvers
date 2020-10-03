use super::structs::{Version, MainBlock, PrereleaseBlock};

pub fn parse_raw_version(raw_version: &str) -> Version{
    let version_without_epoch: String;
    let mut epoch: Option<u8> = None;
    let splitted_epoch_and_tail: Vec<_> = raw_version.split(':').collect();
    if splitted_epoch_and_tail.len() == 2 {
        epoch = Some(splitted_epoch_and_tail[0].parse().unwrap());
        version_without_epoch = splitted_epoch_and_tail[1].to_string();
    } else {
        version_without_epoch = splitted_epoch_and_tail[0].to_string();
    }

    let version_and_prerelease: Vec<_> = version_without_epoch.split('-').collect();
    let main_block: MainBlock = parse_main_block(version_and_prerelease[0].to_string());
    let mut prerelease_block: Option<PrereleaseBlock> = None;
    if version_and_prerelease.len() == 2 {
        let raw_prerelease: String = version_and_prerelease[1].to_string();
        prerelease_block = Some(parse_prerelease(raw_prerelease));
    }
    Version {
        epoch: epoch,
        main: main_block,
        pre_release: prerelease_block,
    }
}

fn parse_main_block(raw_main_block: String) -> MainBlock {
    let mut main_version_numbers: Vec<u32> = Vec::new();
    let mut post_main_letter: Option<char> = None;
    for element in raw_main_block.split('.'){
        let subversion: String = element.to_string();
        let index_without_last_char: usize = subversion.chars().count() - 1;
        if last_char_is_letter(&element) {
            main_version_numbers.push(element[0..index_without_last_char].parse().unwrap());
            post_main_letter = subversion.chars().rev().next();
        } else {
            main_version_numbers.push(element.parse().unwrap());
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

fn parse_prerelease(raw_prerelease: String) -> PrereleaseBlock {
    let step: String;
    let mut post_number: Option<u8> = None;
    let splitted_prerelease: Vec<_> = raw_prerelease.split('.').collect();
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
