#[derive(Debug)]
pub struct AnimationComponent {
    pub frame_count: u8,
    pub current_frame: u8,
    pub frame_rate: u8,
    pub looping: bool,
    pub start_time: f64,
}

impl AnimationComponent {
    pub fn new(frame_count: u8, frame_rate: u8, looping: bool) -> Self {
        Self {
            current_frame: 0,
            frame_count,
            frame_rate,
            looping,
            start_time: 0.0,
        }
    }
}
