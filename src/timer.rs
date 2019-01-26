use simple_stopwatch::Stopwatch;

pub struct Timer {
    stopwatch: Stopwatch,
    lap: f32
}

impl Timer {
    pub fn new() -> Timer {
        let started = Stopwatch::start_new();
        let mut time_now = started.ms();

        Timer {
            stopwatch: started,
            lap: time_now
        }
    }

    pub fn elap_time(&mut self) -> f32 {
        self.stopwatch.ms() / 1000.0
    }

    pub fn frame_time(&mut self) -> f32 {
        let now = self.stopwatch.ms();
        let elapsed = now - self.lap;
        self.lap = now;

        return elapsed
    }
}
