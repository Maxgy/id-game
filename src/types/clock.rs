pub struct Clock {
    hour: u8,
    minute: u8,
    tic: f32,
}

impl Clock {
    pub fn new() -> Self {
        Self {
            hour: 9,
            minute: 30,
            tic: 0.0,
        }
    }

    pub fn print(&self) -> String {
        format!(
            "Time: {}:{}",
            if self.hour < 10 {
                format!("0{}", self.hour)
            } else {
                self.hour.to_string()
            },
            if self.minute < 10 {
                format!("0{}", self.minute)
            } else {
                self.minute.to_string()
            },
        )
    }

    pub fn update(&mut self, dt: f32) {
        if self.tic >= 1.0 {
            self.minute += 1;

            if self.minute >= 60 {
                self.minute = 0;
                self.hour += 1;
            }
            if self.hour >= 24 {
                self.hour = 0;
            }
            self.tic = 0.0;
        } else {
            self.tic += dt;
        }
    }
}
