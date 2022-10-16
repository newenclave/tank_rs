use std::time::Duration;

pub struct Timer {
    duration: Duration,
    time_left: Duration,
    ready: bool,
}

impl Timer {
    pub fn new(dur: Duration) -> Self {
        Self {
            duration: dur,
            time_left: dur,
            ready: dur.is_zero(),
        }
    }

    pub fn from_millis(millis: u64) -> Self {
        let dur = Duration::from_millis(millis);
        Self {
            duration: dur,
            time_left: dur,
            ready: dur.is_zero(),
        }
    }

    pub fn reset(&mut self) {
        self.time_left = self.duration;
        self.ready = self.duration.is_zero();
    }

    pub fn is_max_duration(&self) -> bool {
        self.duration == Duration::MAX
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        if !self.ready {
            if let Some(time_left) = self.time_left.checked_sub(delta) {
                self.time_left = time_left;
            } else {
                self.time_left = Duration::from_millis(0);
                self.ready = true;
            }
        }
        self.ready
    }

    pub fn ready(&self) -> bool {
        self.ready
    }
}
