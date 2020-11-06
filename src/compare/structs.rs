use std::cmp::Ordering;


#[derive(Eq)]
pub struct Version {
    pub epoch: Option<u8>,
    pub main: MainBlock,
    pub pre_release: Option<PrereleaseBlock>,
    pub build: Option<BuildBlock>,
}

#[derive(Eq)]
pub struct MainBlock {
    pub numbers: Vec<u32>,
    pub post_letter: Option<char>,
}

#[derive(Eq)]
pub struct PrereleaseBlock {
    pub step: String,
    pub post_number: Option<u8>,
}

#[derive(Eq)]
pub struct BuildBlock {
    pub number: u8,
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

impl PartialOrd for BuildBlock {
    fn partial_cmp(&self, other: &BuildBlock) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BuildBlock {
    fn cmp(&self, other: &BuildBlock) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialEq for BuildBlock {
    fn eq(&self, other: &BuildBlock) -> bool {
        self.number == other.number
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

        let prerelease_order: Ordering = self.cmp_prerelease(&other);
        if prerelease_order != Ordering::Equal {
            return prerelease_order
        }

        self.cmp_build(&other)
    }
}

impl Version {
    fn cmp_epoch(&self, other: &Version) -> Ordering {
        match [self.epoch, other.epoch] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Greater,
            [None, Some(_)] => Ordering::Less,
            [Some(_), Some(_)] => self.epoch.cmp(&other.epoch),
        }
    }

    fn cmp_prerelease(&self, other: &Version) -> Ordering {
        match [&self.pre_release, &other.pre_release] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Less,
            [None, Some(_)] => Ordering::Greater,
            [Some(_), Some(_)] => self.pre_release.cmp(&other.pre_release),
        }
    }

    fn cmp_build(&self, other: &Version) -> Ordering {
        match [&self.build, &other.build] {
            [None, None] => Ordering::Equal,
            [Some(_), None] => Ordering::Greater,
            [None, Some(_)] => Ordering::Less,
            [Some(_), Some(_)] => self.build.cmp(&other.build),
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
            [Some(_), Some(_)] => self.post_letter.cmp(&other.post_letter),
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
            [Some(_), Some(_)] => self.post_number.cmp(&other.post_number)
        }
    }

    fn cmp_step(&self, other: &PrereleaseBlock) -> Ordering {
        match [self.step.len(), other.step.len()] {
            [0, x] if x > 0 => return Ordering::Greater,
            [x, 0] if x > 0 => return Ordering::Less,
            _ => self.step.cmp(&other.step),
        }
    }
}