use sdl2::{event::Event, keyboard::Keycode, EventPump};

#[derive(Debug, Default)]
pub struct InputResult {
    pub should_quit: bool,
    pub w: bool,
    pub a: bool,
    pub s: bool,
    pub d: bool,
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
    pub space: bool,
}

impl InputResult {
    pub fn new() -> InputResult {
        InputResult {
            ..Default::default()
        }
    }
}

pub struct InputProcessor {
    event_pump: EventPump,
    result: InputResult,
}

impl InputProcessor {
    pub fn new(event_pump: EventPump) -> Self {
        Self {
            event_pump,
            result: InputResult::new(),
        }
    }

    pub fn process(&mut self) -> &InputResult {
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
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.result.up = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.result.left = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => self.result.down = true,
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.result.right = true,
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
                Event::KeyUp {
                    keycode: Some(Keycode::Up),
                    ..
                } => self.result.up = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => self.result.left = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => self.result.down = false,
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => self.result.right = false,
                _ => (),
            };
        }

        &self.result
    }
}
