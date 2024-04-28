use sdl2::{event::Event, keyboard::Keycode, EventPump};

pub struct InputResult {
    pub should_quit: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub space: bool,
}

impl Default for InputResult {
    fn default() -> InputResult {
        InputResult::new()
    }
}

impl InputResult {
    pub fn new() -> InputResult {
        InputResult {
            should_quit: false,
            w: false,
            a: false,
            s: false,
            d: false,
            space: false,
        }
    }
}

pub struct InputController {
    event_pump: EventPump,
    result: InputResult,
}

impl InputController {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            result: InputResult::new(),
        }
    }

    pub fn process(&mut self) {
        #[allow(clippy::never_loop)]
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.result.should_quit = true,
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => self.result.w = true,
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => self.result.a = true,
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => self.result.s = true,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => self.result.d = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.result.space = true,
                Event::KeyUp {
                    keycode: Some(Keycode::W),
                    ..
                } => self.result.w = false,
                Event::KeyUp {
                    keycode: Some(Keycode::A),
                    ..
                } => self.result.a = false,
                Event::KeyUp {
                    keycode: Some(Keycode::S),
                    ..
                } => self.result.s = false,
                Event::KeyUp {
                    keycode: Some(Keycode::D),
                    ..
                } => self.result.d = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Space),
                    ..
                } => self.result.space = false,
                _ => (),
            };
        }
    }

    pub fn result(&self) -> &InputResult {
        &self.result
    }
}
