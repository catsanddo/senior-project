use std::time;

pub struct Clock {
    now: time::Instant,
    delta_time: time::Duration,
    fps: u8,
}

impl Clock {
    pub fn new(fps: u8) -> Self {
        Self {
            now: time::Instant::now(),
            delta_time: time::Duration::new(0, 0),
            fps: fps,
        }
    }

    pub fn tick(&mut self) {
        // Check if we should sleep and for how long, then sleep
        if self.now.elapsed().as_millis() < ((1.0 / self.fps as f32) * 1000.0) as u128 {
            std::thread::sleep(time::Duration::from_millis((((1.0 / self.fps as f32) * 1000.0) as u128 - self.now.elapsed().as_millis()) as u64));
        }

        // Update delta_time
        self.delta_time = self.now.elapsed();
        // Update instant
        self.now = time::Instant::now();
    }
}
