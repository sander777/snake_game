pub use glutin_window::GlutinWindow;
pub use opengl_graphics::{GlGraphics, OpenGL};
pub use piston::event_loop::{EventSettings, Events};
pub use piston::input::{
    Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent,
};
pub use piston::window::WindowSettings;

use food;
use snake;

pub struct Ctx {
    pub field_size: (i32, i32),
    pub snake: Vec<(i32, i32)>,
    pub food_pos: (i32, i32),
    pub size: i32,
}

pub struct SnakeApp {
    gl: GlGraphics,
    snake: snake::Snake,
    food: food::Food,
    ground: [f32; 4],
    size: i32,
    field_size: (i32, i32),
    delta: f64,
    state: i8,
    upd_dlt: f64,
}

impl SnakeApp {
    pub fn new(opengl: OpenGL) -> SnakeApp {
        SnakeApp {
            gl: GlGraphics::new(opengl),
            snake: snake::Snake::new(),
            ground: [0.1, 0.1, 0.1, 1.0],
            delta: 0.0,
            food: food::Food::new(),
            size: 50,
            field_size: (0, 0),
            state: 0,
            upd_dlt: 0.05,
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
        self.food.change_color(color);
        self
    }

    pub fn change_size(mut self, size: i32) -> Self {
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
            snake: self.snake.body_ref().clone(),
            food_pos: self.food.pos,
            field_size: self.field_size,
        }
    }

    pub fn render(&mut self, args: RenderArgs) {
        use graphics::*;

        let ground = self.ground;

        self.field_size = (
            args.window_size[0] as i32 / self.ctx().size as i32,
            args.window_size[1] as i32 / self.ctx().size as i32,
        );

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(ground, gl);
        });
        let mut ctx = self.ctx();
        self.snake.render(args, &mut self.gl, &mut ctx);
        self.food.render(args, &mut self.gl, &ctx)
    }

    pub fn update(&mut self, upd_args: UpdateArgs, button: &mut Option<Button>) -> i8 {
        self.delta += upd_args.dt;
        if self.state == 0 && self.delta > self.upd_dlt {
            self.delta = 0.0;
            let mut ctx = self.ctx();
            self.snake.update(upd_args, *button, &ctx);
            if self.food.update(upd_args, &mut ctx) {
                self.snake.grow();
                println!("{}", self.snake.body_ref().len());
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
