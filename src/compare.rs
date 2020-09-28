use std::cmp::Ordering;


#[derive(Eq)]
struct Version {
    epoch: Option<u8>,
    main: MainBlock,
    pre_release: Option<PrereleaseBlock>,
}

#[derive(Eq)]
struct MainBlock {
    numbers: Vec<u32>,
    post_letter: Option<char>,
}

#[derive(Eq)]
struct PrereleaseBlock {
    step: String,
    post_number: Option<u8>,
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for MainBlock {
    fn partial_cmp(&self, other: &MainBlock) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for PrereleaseBlock {
    fn partial_cmp(&self, other: &PrereleaseBlock) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        let epoch_order: Ordering = self.cmp_epoch(other);
        if epoch_order != Ordering::Equal {
            return epoch_order
        }

        let main_order: Ordering = self.main.cmp(&other.main);
        if main_order != Ordering::Equal {
            return main_order
        }

        match [&self.pre_release, &other.pre_release] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Less,
            [None, Some(_)] => Ordering::Greater,
            [Some(_), Some(_)] => {
               self.pre_release.cmp(&other.pre_release)
            }
        }
    }
}

impl Version {
    fn cmp_epoch(&self, other: &Version) -> Ordering {
        match [self.epoch, other.epoch] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Greater,
            [None, Some(_)] => Ordering::Less,
            [Some(_), Some(_)] => {
                self.epoch.cmp(&other.epoch)
            }
        }
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Version) -> bool {
        self.main == other.main && self.pre_release == other.pre_release
    }
}

impl PartialEq for MainBlock {
    fn eq(&self, other: &MainBlock) -> bool {
        self.numbers == other.numbers && self.post_letter == other.post_letter
    }
}

impl PartialEq for PrereleaseBlock {
    fn eq(&self, other: &PrereleaseBlock) -> bool {
        self.step == other.step && self.post_number == other.post_number
    }
}

impl Ord for MainBlock {
    fn cmp(&self, other: &MainBlock) -> Ordering {
        let order: Ordering = self.cmp_numbers(other);
        if order != Ordering::Equal {
            return order
        }

        self.cmp_post_letter(other)
    }
}

impl MainBlock {
    fn cmp_numbers(&self, other: &MainBlock) -> Ordering {
        self.numbers.cmp(&other.numbers)
    }

    fn cmp_post_letter(&self, other: &MainBlock) -> Ordering {
        match [self.post_letter, other.post_letter] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Greater,
            [None, Some(_)] => Ordering::Less,
            [Some(_), Some(_)] => {
                self.post_letter.cmp(&other.post_letter)
            }
        }
    }
}


impl Ord for PrereleaseBlock {
    fn cmp(&self, other: &PrereleaseBlock) -> Ordering {
        let order: Ordering = self.cmp_step(other);
        if order != Ordering::Equal {
            return order
        }

        return self.cmp_post_number(other);
    }
}

impl PrereleaseBlock {
    fn cmp_post_number(&self, other: &PrereleaseBlock) -> Ordering {
        match [self.post_number, other.post_number] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Greater,
            [None, Some(_)] => Ordering::Less,
            [Some(_), Some(_)] => {
                self.post_number.cmp(&other.post_number)
            }
        }
    }

    fn cmp_step(&self, other: &PrereleaseBlock) -> Ordering {
        match [self.step.len(), other.step.len()] {
            [0, x] if x > 0 => return Ordering::Greater,
            [x, 0] if x > 0 => return Ordering::Less,
            _ => {
                self.step.cmp(&other.step)
            }
        }
    }
}


pub fn compare(raw_version_a: &str, raw_version_b: &str)-> Ordering{
    let version_a: Version = parse_raw_version(raw_version_a);
    let version_b: Version = parse_raw_version(raw_version_b);

    version_a.cmp(&version_b)
}

fn parse_raw_version(raw_version: &str) -> Version{
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
    return Version {
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
        let last_char_is_letter: bool = ! element.chars().last().unwrap().is_digit(10);
        if last_char_is_letter {
            main_version_numbers.push(element[0..index_without_last_char].parse().unwrap());
            post_main_letter = subversion.chars().rev().next();
        } else {
            main_version_numbers.push(element.parse().unwrap());
        }
    }
    return MainBlock {
        numbers: main_version_numbers,
        post_letter: post_main_letter,
    }
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
    return PrereleaseBlock {
        step: step,
        post_number: post_number,
    }
}
