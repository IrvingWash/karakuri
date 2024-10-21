use sdl2::TimerSubsystem;

const MILLISECONDS_PER_MINUTE: u32 = 1000;
const MINUTES_PER_MILLISECOND: f64 = 1. / MILLISECONDS_PER_MINUTE as f64;

pub struct FpsController {
    time_previous_frame: u32,
    timer: TimerSubsystem,
    milliseconds_per_frame: u32,
    seconds_per_frame_cap: f64,
}

impl FpsController {
    pub fn new(timer: TimerSubsystem, target_fps: u32, min_update_fps: u32) -> Self {
        Self {
            timer,
            time_previous_frame: 0,
            milliseconds_per_frame: MILLISECONDS_PER_MINUTE / target_fps,
            seconds_per_frame_cap: 1. / min_update_fps as f64,
        }
    }

    pub fn cap_framerate(&mut self) -> f64 {
        let time_to_wait = self
            .milliseconds_per_frame
            .saturating_sub(self.timer.ticks() - self.time_previous_frame);
        self.timer.delay(time_to_wait);

        let mut delta_time =
            (self.timer.ticks() - self.time_previous_frame) as f64 * MINUTES_PER_MILLISECOND;
        if delta_time > self.seconds_per_frame_cap {
            delta_time = self.seconds_per_frame_cap
        }

        self.time_previous_frame = self.timer.ticks();

        delta_time
    }
}
