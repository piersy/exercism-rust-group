#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let extra_hours = minutes / 60;
        let mut hours = (hours + extra_hours) % 24;
        if hours < 0 {
            hours = hours + 24;
        }
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes = minutes + 60;
        }

        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let extra_hours = minutes / 60;
        Clock {
            hours: self.hours + extra_hours,
            minutes: minutes % 60,
        }

        // unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "({}, {})", self.hours, self.minutes)
//     }
// }
