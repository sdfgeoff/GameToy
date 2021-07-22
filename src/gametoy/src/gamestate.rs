use chrono::{Datelike, Timelike};


pub struct GameState {
    // Time since the program began
    pub time_since_start: f64,

    // Time it took to render the previous frame
    pub time_delta: f64,

    // Date as year, month, day, time-in-seconds
    pub date: [u32; 4],

    // Time the last frame was rendered - used to calculate dt
    prev_render_time: f64,
}


impl GameState {
    pub fn new() -> Self {
        Self {
            time_since_start: 0.0,
            time_delta: 0.0,
            date: [0, 0, 0, 0],
            prev_render_time: 0.0
        }
    }

    pub fn update_times(&mut self, time_since_unix_epoch: f64) {
        let dt = if self.prev_render_time == 0.0 {
            0.016
        } else {
            // Cap the dt at 0.0 - time should not move backwards
            f64::max(time_since_unix_epoch - self.prev_render_time, 0.0)
        };

        self.prev_render_time = time_since_unix_epoch;
        self.time_since_start += dt;
        self.time_delta = dt;

        let secs = time_since_unix_epoch.floor() as i64;
        let datetime = chrono::NaiveDateTime::from_timestamp(secs, 0);
        self.date = [
            datetime.year() as u32,
            datetime.month(),
            datetime.day(),
            datetime.num_seconds_from_midnight(),
        ];
    }
}