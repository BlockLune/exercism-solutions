#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock { hours, minutes };
        clock.flatten();
        clock
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut clock = Clock {
            hours: self.hours,
            minutes: self.minutes + minutes,
        };
        clock.flatten();
        clock
    }

    fn flatten(&mut self) {
        while self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }
        while self.hours < 0 {
            self.hours += 24;
        }
        self.hours = (self.hours + self.minutes / 60) % 24;
        self.minutes %= 60;
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours * 60 + self.minutes == other.hours * 60 + other.minutes
    }
}
