#[derive(Clone)]
pub struct Digits {
    num: u32,
    mask: u32,
}

impl Digits {
    pub fn new(num: u32) -> Self {
        let mask = match num {
            0 => 1,
            n => 10_f64.powf((n as f64).log10().floor()) as u32,
        };

        Digits { mask, num }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask == 0 {
            return None;
        }

        let digit = self.num / self.mask % 10;
        self.mask /= 10;

        Some(digit)
    }
}

// just playing with iterators 🤡 (see https://docs.rs/digits_iterator/0.1.0/digits_iterator/)
pub fn is_armstrong_number(num: u32) -> bool {
    let digits = Digits::new(num);
    let p = digits.clone().count() as u32;
    digits.map(|n| n.pow(p)).sum::<u32>() == num
}
