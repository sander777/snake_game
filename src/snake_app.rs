pub use glutin_window::GlutinWindow;
pub use opengl_graphics::{GlGraphics, OpenGL};
pub use piston::event_loop::{EventSettings, Events};
pub use piston::input::{
    Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
pub use piston::window::WindowSettings;
pub use rodio::{Decoder, Device, Source};
pub use std::fs::File;
pub use std::io::BufReader;

use food;
use snake;

pub struct Ctx {
    pub field_size: (u32, u32),
    pub snake: Box<Vec<(i32, i32)>>,
    pub food_pos: Vec<(i32, i32)>,
    pub size: u32,
}

pub struct SnakeApp {
    gl: GlGraphics,
    snake: snake::Snake,
    food: Vec<food::Food>,
    ground: [f32; 4],
    size: u32,
    field_size: (u32, u32),
    delta: f64,
    state: i8,
    upd_dlt: f64,
    device: Device,
}

impl SnakeApp {
    pub fn new(opengl: OpenGL) -> SnakeApp {
        let device = rodio::default_output_device().unwrap();

        SnakeApp {
            gl: GlGraphics::new(opengl),
            snake: snake::Snake::new(),
            ground: [0.1, 0.1, 0.1, 1.0],
            delta: 0.0,
            food: (0..1).map(|_| food::Food::new()).collect(),
            size: 50,
            field_size: (25, 25),
            state: 0,
            upd_dlt: 0.05,
            device: device,
        }
    }

    pub fn snake_color(mut self, first_color: [f32; 4], second_color: [f32; 4]) -> Self {
        self.snake.change_color(first_color, second_color);
        self
    }

    pub fn background_color(mut self, color: [f32; 4]) -> Self {
        self.ground = color;
        self
    }

    pub fn food_color(mut self, color: [f32; 4]) -> Self {
        for i in &mut self.food {
            i.change_color(color);
        }
        self
    }

    pub fn init_field(mut self, size: (u32, u32)) -> Self {
        self.field_size = size;
        self
    }

    pub fn food_count(mut self, n: usize) -> Self {
        let mut new_food = Vec::<food::Food>::with_capacity(n);
        for _ in 0..n {
            let mut food = food::Food::new();
            let mut ctx = self.ctx();
            food.new_pos(&mut ctx);
            new_food.push(food);
        }
        self.food = new_food;
        self
    }

    pub fn change_size(mut self, size: u32) -> Self {
        self.size = size;
        self
    }

    pub fn update_delta(mut self, dlt: f64) -> Self {
        self.upd_dlt = dlt;
        self
    }

    fn ctx(self: &mut Self) -> Ctx {
        Ctx {
            size: self.size,
            snake: self.snake.body_ref(),
            food_pos: self.food.iter().map(|f| (f.get_pos())).collect(),
            field_size: self.field_size,
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        let ground = self.ground;

        self.field_size = (
            args.window_size[0] as u32 / self.ctx().size as u32,
            args.window_size[1] as u32 / self.ctx().size as u32,
        );

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(ground, gl);
        });
        let mut ctx = self.ctx();
        self.snake.render(args, &mut self.gl, &mut ctx);
        for i in &mut self.food {
            i.render(args, &mut self.gl, &ctx)
        }
    }

    pub fn update(
        &mut self,
        upd_args: UpdateArgs,
        button: &mut Option<Button>,
        score: &mut u32,
    ) -> i8 {
        self.delta += upd_args.dt;
        *score = self.snake.body_ref().len() as u32;
        if self.state == 0 && self.delta > self.upd_dlt {
            self.delta = 0.0;
            let mut ctx = self.ctx();
            self.snake.update(upd_args, *button, &ctx);
            for i in &mut self.food {
                if i.update(upd_args, &mut ctx) {
                    self.snake.grow();
                    let file = File::open("resources/EAT_Sound.wav").unwrap();
                    let source = rodio::Decoder::new(BufReader::new(file))
                        .unwrap()
                        .convert_samples();
                    rodio::play_raw(&self.device, source);
                }
            }
            *button = None;
        }
        if !self.snake.is_alive() {
            self.state = -1;
            self.snake
                .change_color([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, 1.0]);
        }
        self.state
    }
}
