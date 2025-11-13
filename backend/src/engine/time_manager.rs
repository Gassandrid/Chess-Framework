use std::time::Duration;

pub struct TimeManager {
    pub white_time: Duration,
    pub black_time: Duration,
    pub white_inc: Duration,
    pub black_inc: Duration,
    pub moves_to_go: Option<u16>,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            white_time: Duration::from_secs(300),
            black_time: Duration::from_secs(300),
            white_inc: Duration::ZERO,
            black_inc: Duration::ZERO,
            moves_to_go: None,
        }
    }

    pub fn calculate_time_for_move(&self, is_white: bool) -> Duration {
        let (time, inc) = if is_white {
            (self.white_time, self.white_inc)
        } else {
            (self.black_time, self.black_inc)
        };

        let time_ms = time.as_millis() as f64;
        let inc_ms = inc.as_millis() as f64;

        let allocated = if let Some(moves) = self.moves_to_go {
            // If we know moves to go, divide time accordingly
            (time_ms / moves as f64) + inc_ms
        } else {
            // Otherwise, use a fraction of remaining time plus increment
            // Assume ~40 moves remaining
            (time_ms / 40.0) + inc_ms
        };

        // Don't use more than 1/3 of remaining time
        let max_time = time_ms / 3.0;
        let allocated = allocated.min(max_time);

        // Minimum time
        let allocated = allocated.max(100.0);

        Duration::from_millis(allocated as u64)
    }
}

impl Default for TimeManager {
    fn default() -> Self {
        Self::new()
    }
}
