#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut extra_hours = minutes / 60;
        let mut minutes = minutes % 60;
        if minutes < 0 {
            minutes = minutes + 60;
            extra_hours = extra_hours - 1;
        }
        let mut hours = (hours + extra_hours) % 24;
        if hours < 0 {
            hours = hours + 24;
        }

        Clock {
            hours: hours,
            minutes: minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
