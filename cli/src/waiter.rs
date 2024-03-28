use std::{
    thread,
    time::{Duration, Instant},
};

pub struct Waiter {
    target: Duration,
    start_time: Instant,
    end_time: Instant,
}

impl Waiter {
    pub fn new(target: Duration) -> Self {
        let now = Instant::now();

        Self {
            target,
            start_time: now,
            end_time: now,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now()
    }

    pub fn end(&mut self) {
        self.end_time = Instant::now()
    }

    pub fn cycle(&self) {
        let delta = self.end_time - self.start_time;
        if self.target > delta {
            thread::sleep(self.target - delta);
        }
    }
}
