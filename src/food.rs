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
        if ctx.field_size.0 as i32 * ctx.field_size.1 as i32
            - ctx.snake.len() as i32
            - ctx.food_pos.len() as i32
            + 1
            > 0
        {
            let mut field = Vec::<(i32, i32)>::new();
            for y in 0..ctx.field_size.1 {
                field.append(
                    &mut (0..ctx.field_size.0)
                        .map(|x| (x as i32, y as i32))
                        .collect::<Vec<_>>(),
                )
            }
            for i in ctx.snake.iter() {
                let index = field.iter().position(|x| x == i);
                match index {
                    Some(index) => {
                        field.remove(index);
                    }
                    None => {}
                }
            }
            for i in ctx.food_pos.iter() {
                let index = field.iter().position(|x| x == i);
                match index {
                    Some(index) => {
                        field.remove(index);
                    }
                    None => {}
                }
            }
            let mut rng = thread_rng();
            self.pos = field[rng.gen_range(0, field.len())];
        } else {
            self.pos = (-1, -1);
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
