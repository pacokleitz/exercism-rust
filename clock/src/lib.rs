use std::fmt;

const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i64,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.minutes / HOUR;
        let minutes = self.minutes % HOUR;

        return write!(
            f,
            "{}:{}",
            format!("{:02}", hours),
            format!("{:02}", minutes)
        );
    }
}

impl Clock {
    pub fn new(hours: i64, minutes: i64) -> Self {
        Clock {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY,
        }
    }

    pub fn add_minutes(&self, minutes: i64) -> Self {
        return Clock::new(0, self.minutes + minutes);
    }
}
