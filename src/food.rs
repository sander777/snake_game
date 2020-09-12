use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use snake_app::Ctx;

use rand::prelude::*;

pub struct Food {
    color: [f32; 4],
    pub pos: (i32, i32),
}

impl Food {
    pub fn new() -> Food {
        Food {
            pos: (5, 5),
            color: [1.0, 1.0, 0.0, 1.0],
        }
    }
    pub fn new_pos(self: &mut Self, ctx: &mut Ctx) {
        loop {
            let mut rng = thread_rng();
            let x = rng.gen_range(0, ctx.field_size.0);
            let y = rng.gen_range(0, ctx.field_size.1);
            if !ctx.snake.contains(&(x as i32, y as i32)) && !ctx.food_pos.contains(&(x as i32, y as i32)){
                self.pos = (x as i32, y as i32);
                break;
            }
        }
    }

    pub fn render(self: &mut Self, args: RenderArgs, gl: &mut GlGraphics, ctx: &Ctx) {
        use graphics::*;

        gl.draw(args.viewport(), |c, gl| {
            let square = rectangle::square(0.0, 0.0, ctx.size as f64);
            let transform = c.transform.trans(
                self.pos.0 as f64 * ctx.size as f64,
                self.pos.1 as f64 * ctx.size as f64,
            );
            rectangle(self.color, square, transform, gl);
        });
    }

    pub fn update(self: &mut Self, _upd_args: UpdateArgs, ctx: &mut Ctx) -> bool {
        let mut is_growing = false;
        if ctx.snake[0] == self.pos {
            self.new_pos(ctx);
            is_growing = true;
        }
        is_growing
    }

    pub fn change_color(self: &mut Self, color: [f32; 4]) {
        self.color = color;
    }

    pub fn get_pos(self: &Self) -> (i32, i32) {
        self.pos.clone()
    }
}
