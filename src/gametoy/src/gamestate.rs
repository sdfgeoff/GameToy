use chrono::{Datelike, Timelike};


pub struct GameState {
    /// Time since the program began
    pub time_since_start: f64,

    /// Time it took to render the previous frame
    pub time_delta: f64,

    /// Date as year, month, day, time-in-seconds
    pub date: [u32; 4],

    /// Time the last frame was rendered - used to calculate dt
    prev_render_time: f64,

    /// The state of the keyboard. Each key has three entries:
    ///  - Currently Pressed
    ///  - Edge ("just pressed")
    ///  - Toggle (changes state each time the key is pressed)
    /// They are arranged:
    ///
    /// - keycode = currently_pressed
    /// - 256 + keycode = edge
    /// - 512 + keycode = toggle
    ///
    /// This is to align with the shadertoy keyboard representation.
    pub keys: [i8; 768],

    /// Set to true when the keys array has been changed.
    pub keys_dirty: bool,
}


impl GameState {
    pub fn new() -> Self {
        Self {
            time_since_start: 0.0,
            time_delta: 0.0,
            date: [0, 0, 0, 0],
            prev_render_time: 0.0,
            keys: [0;768],
            keys_dirty: false,
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

    pub fn set_key_state(&mut self, keycode: usize, state: bool) {
        if state != (self.keys[keycode] == 1) {
            // Pressed/held state
            self.keys[keycode] = state as i8;

            // Edge state
            if state == true {
                self.keys[256+keycode] = 1;
            } else {
                self.keys[256+keycode] = -1;
            }

            // Toggle
            if state == true {
                if self.keys[512+keycode] == 0 {
                    self.keys[512+keycode] = 1;
                } else {
                    self.keys[512+keycode] = 0;
                }
            }

            self.keys_dirty = true;
        }
    }

    /// Keys have an edge detect trigger. This needs to be run at the end of each frame
    /// to ensure that the edge-trigger is correctly de-set for the next frame
    pub fn update_key_tick(&mut self) {
        for keycode in 0..256 {
            if self.keys[256+keycode] != 0 {
                self.keys[256+keycode] = 0;
                self.keys_dirty = true;
            }
        }
    }

    /// There is a "dirty" flag used to indicate if the keys have changed. This clears
    /// the flag. It should be run once at the end of the frame.
    pub fn clear_keys_dirty(&mut self) {
        self.keys_dirty = false;
    }
}