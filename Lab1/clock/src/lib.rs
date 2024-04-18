use  std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    // internally we store the time in minutes, so that we can easily perform arithmetic operations
    // please note that all date/time libraries use this approach, storing time in simple units
    minutes: i32
}

impl Clock {
    
    // adjusting sign of minutes to be positive
    fn from_minutes(minutes: i32) -> Self {
        if minutes < 0 {
            // we need to add 24 hours to the negative minutes
            Clock { minutes: (minutes % (60*24)) + (60*24) }
        } else {
            Clock { minutes: minutes % (60*24) }
        }
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_minutes(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // with internal time in minutes it's trivial to add minutes
        Clock::from_minutes(self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        // we convert to hours and minutes only before display
        let hours = self.minutes / 60;
        let minutes = self.minutes % 60;
        format!("{:02}:{:02}", hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
