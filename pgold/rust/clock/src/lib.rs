use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn mins(hours: i32, minutes: i32) -> i32 {
    60 * hours + minutes
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_mins(mins(hours, minutes))
    }

    pub fn from_mins(minutes: i32) -> Self {
        dbg!(minutes);
        let hours_rem = (minutes / 60) % 24;
        let min_rem = minutes.rem_euclid(60);
        Clock {
            hours: if minutes < 0 {
                hours_rem + 23 + ((min_rem == 0) as i32)
            } else {
                hours_rem
            },
            minutes: min_rem,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from_mins(mins(self.hours, self.minutes) + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
