use chrono::{Duration, NaiveTime};
use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Clock {
    time: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_effect_on_hours = minutes / 60 + if minutes < 0 && minutes % 60 < 0 { -1 } else { 0 };
        let normalized_hours: u32 = (((hours + minutes_effect_on_hours) % 24 + 24) % 24) as u32;
        let normalized_minutes: u32 = (((minutes % 60) + 60) % 60) as u32;

        Clock {
            time: NaiveTime::from_hms(normalized_hours, normalized_minutes, 0),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            time: self.time + Duration::minutes(minutes as i64)
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.time.format("%H:%M").to_string())
    }
}