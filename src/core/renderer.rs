use crate::{
    math::Vector2,
    utils::{Color, Resolution},
};
use sdl2::{
    gfx::primitives::DrawRenderer, pixels::Color as SdlColor, render::Canvas, video::Window, Sdl,
};

const DRAW_FAILURE_MESSAGE: &str = "Failed to draw";

pub struct Renderer {
    canvas: Canvas<Window>,
    clear_color: SdlColor,
}

impl Renderer {
    pub fn new(sdl: &Sdl, title: &str, resolution: Resolution, clear_color: Color) -> Self {
        let window = Self::open_window(sdl, title, resolution);

        Self {
            canvas: Self::create_canvas(window),
            clear_color: SdlColor::RGBA(
                clear_color.red,
                clear_color.green,
                clear_color.blue,
                clear_color.alpha,
            ),
        }
    }

    pub fn start_frame(&mut self) {
        self.clear_screen();
    }

    pub fn finish_frame(&mut self) {
        self.canvas.present();
    }

    pub fn resolution(&self) -> Resolution {
        let (width, height) = self.canvas.window().size();

        Resolution::new(width, height)
    }

    pub fn _line(&mut self, a: &Vector2, b: &Vector2, color: &Color) {
        self.canvas
            .line(
                a.x as i16,
                a.y as i16,
                b.x as i16,
                b.y as i16,
                color.to_tuple(),
            )
            .expect(DRAW_FAILURE_MESSAGE)
    }

    pub fn _circle(&mut self, position: &Vector2, radius: f64, angle: f64, color: &Color) {
        self.canvas
            .circle(
                position.x as i16,
                position.y as i16,
                radius as i16,
                color.to_tuple(),
            )
            .expect(DRAW_FAILURE_MESSAGE);

        self.canvas
            .line(
                position.x as i16,
                position.y as i16,
                (position.x + angle.cos() * radius) as i16,
                (position.y + angle.sin() * radius) as i16,
                color.to_tuple(),
            )
            .expect(DRAW_FAILURE_MESSAGE);
    }

    pub fn _filled_circle(&mut self, position: &Vector2, radius: f64, color: &Color) {
        self.canvas
            .filled_circle(
                position.x as i16,
                position.y as i16,
                radius as i16,
                color.to_tuple(),
            )
            .expect(DRAW_FAILURE_MESSAGE);
    }

    pub fn _rectangle(&mut self, position: &Vector2, size: &Vector2, color: &Color) {
        let half_width = size.x * 0.5;
        let half_height = size.y * 0.5;
        let color_tuple = color.to_tuple();

        self.canvas
            .line(
                (position.x - half_width) as i16,
                (position.y - half_height) as i16,
                (position.x + half_width) as i16,
                (position.y - half_height) as i16,
                color_tuple,
            )
            .expect(DRAW_FAILURE_MESSAGE);

        self.canvas
            .line(
                (position.x + half_width) as i16,
                (position.y - half_height) as i16,
                (position.x + half_width) as i16,
                (position.y + half_height) as i16,
                color_tuple,
            )
            .expect(DRAW_FAILURE_MESSAGE);

        self.canvas
            .line(
                (position.x + half_width) as i16,
                (position.y + half_height) as i16,
                (position.x - half_width) as i16,
                (position.y + half_height) as i16,
                color_tuple,
            )
            .expect(DRAW_FAILURE_MESSAGE);

        self.canvas
            .line(
                (position.x - half_width) as i16,
                (position.y + half_height) as i16,
                (position.x - half_width) as i16,
                (position.y - half_height) as i16,
                color_tuple,
            )
            .expect(DRAW_FAILURE_MESSAGE);
    }

    pub fn filled_rectangle(&mut self, position: &Vector2, size: &Vector2, color: &Color) {
        let half_width = size.x * 0.5;
        let half_height = size.y * 0.5;

        self.canvas
            .box_(
                (position.x - half_width) as i16,
                (position.y - half_height) as i16,
                (position.x + half_width) as i16,
                (position.y + half_height) as i16,
                color.to_tuple(),
            )
            .expect(DRAW_FAILURE_MESSAGE)
    }

    pub fn _polygon(&mut self, position: &Vector2, vertices: &[Vector2], color: &Color) {
        let color_tuple = color.to_tuple();

        for i in 0..vertices.len() {
            let current_index = i;
            let next_index = (i + 1) % vertices.len();

            self.canvas
                .line(
                    (vertices[current_index].x) as i16,
                    (vertices[current_index].y) as i16,
                    (vertices[next_index].x) as i16,
                    (vertices[next_index].y) as i16,
                    color_tuple,
                )
                .expect(DRAW_FAILURE_MESSAGE);
        }

        self.canvas
            .filled_circle((position.x) as i16, (position.y) as i16, 1, color_tuple)
            .expect(DRAW_FAILURE_MESSAGE);
    }

    pub fn _filled_polygon(&mut self, position: &Vector2, vertices: &Vec<Vector2>, color: &Color) {
        let mut vx: Vec<i16> = Vec::new();
        let mut vy: Vec<i16> = Vec::new();

        for vertex in vertices {
            vx.push(vertex.x as i16);
            vy.push(vertex.y as i16);
        }

        self.canvas
            .filled_polygon(&vx, &vy, color.to_tuple())
            .expect(DRAW_FAILURE_MESSAGE);

        self.canvas
            .filled_circle(
                (position.x) as i16,
                (position.y) as i16,
                1,
                Color::RED.to_tuple(),
            )
            .expect(DRAW_FAILURE_MESSAGE);
    }

    fn clear_screen(&mut self) {
        self.canvas.set_draw_color(self.clear_color);

        self.canvas.clear();
    }

    fn open_window(sdl: &Sdl, title: &str, resolution: Resolution) -> Window {
        let video_subsystem = sdl.video().unwrap_or_else(|e| {
            panic!("Failed to get SDL2 video subsystem: {}", e);
        });

        video_subsystem
            .window(title, resolution.width, resolution.height)
            .position_centered()
            .borderless()
            .fullscreen()
            .build()
            .unwrap_or_else(|e| {
                panic!("Failed to create SDL2 window: {}", e);
            })
    }

    fn create_canvas(window: Window) -> Canvas<Window> {
        window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .unwrap_or_else(|e| {
                panic!("Failed to create SDL2 canvas: {}", e);
            })
    }
}
