use std::time::{Instant, Duration};

#[derive(Debug)]
pub struct Timer {
    last_time: Instant,
    current_time: Instant
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            last_time: now,
            current_time: now
        }
    }

    pub fn update(&mut self) {
        (self.last_time, self.current_time) = (self.current_time, Instant::now())
    }

    pub fn delta(&self) -> Duration {
        self.current_time - self.last_time
    }
}


