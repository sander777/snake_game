use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

use snake_app::Ctx;
#[derive(Copy, Clone, PartialEq)]
enum DIR {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl DIR {
    fn is_opposite(self: &Self, other: &DIR) -> bool {
        *other == DIR::UP && *self == DIR::DOWN
            || *other == DIR::LEFT && *self == DIR::RIGHT
            || *other == DIR::DOWN && *self == DIR::UP
            || *other == DIR::RIGHT && *self == DIR::LEFT
    }
}

pub struct Snake {
    first_color: [f32; 4],
    second_color: [f32; 4],
    dir: DIR,
    body: Box<Vec<(i32, i32)>>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            first_color: [0.0, 1.0, 1.0, 1.0],
            second_color: [1.0, 1.0, 0.0, 1.0],
            dir: DIR::RIGHT,
            body: Box::from(vec![(0, 0)]),
        }
    }
    pub fn render(self: &mut Self, args: RenderArgs, gl: &mut GlGraphics, ctx: &Ctx) {
        use graphics::*;
        gl.draw(args.viewport(), |c, gl| {
            let square = rectangle::square(0.0, 0.0, ctx.size as f64);
            let mut t = self.body.len() as f32;
            for i in &(*self.body) {
                let color = trans_color(
                    self.first_color,
                    self.second_color,
                    t / self.body.len() as f32,
                );
                t -= 1.0;
                let transform = c
                    .transform
                    .trans(i.0 as f64 * ctx.size as f64, i.1 as f64 * ctx.size as f64);
                rectangle(color, square, transform, gl);
            }
        });
    }

    pub fn update(self: &mut Self, _upd_args: UpdateArgs, button: Option<Button>, ctx: &Ctx) {
        for i in (1..self.body.len()).rev() {
            let temp = self.body[i - 1].clone();
            self.body[i] = temp;
        }
        let mut new_dir: DIR = self.dir;
        match button {
            Some(button) => match button {
                Button::Keyboard(key) => {
                    new_dir = match key {
                        Key::W => DIR::UP,
                        Key::D => DIR::RIGHT,
                        Key::S => DIR::DOWN,
                        Key::A => DIR::LEFT,
                        _ => self.dir,
                    };
                }
                _ => {}
            },
            _ => {}
        }
        if !self.dir.is_opposite(&new_dir) {
            self.dir = new_dir
        }
        let vec = match self.dir {
            DIR::RIGHT => (1, 0),
            DIR::UP => (0, -1),
            DIR::DOWN => (0, 1),
            DIR::LEFT => (-1, 0),
        };

        self.body[0].0 += vec.0;
        self.body[0].1 += vec.1;
        self.body[0].0 %= ctx.field_size.0 as i32;
        self.body[0].1 %= ctx.field_size.1 as i32;
        if self.body[0].0 < 0 {
            self.body[0].0 += ctx.field_size.0 as i32
        }
        if self.body[0].1 < 0 {
            self.body[0].1 += ctx.field_size.1 as i32
        }
    }
    pub fn is_alive(self: &mut Self) -> bool {
        let l = self.body.len();
        for i in 0..l {
            for j in 0..l {
                if self.body[i] == self.body[j] && j != i {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn body_ref(self: &Self) -> Box<Vec<(i32, i32)>> {
        let mut res = unsafe { Box::<Vec<(i32, i32)>>::new_zeroed().assume_init() };
        res.clone_from(&self.body);
        res
    }

    pub fn grow(self: &mut Self) {
        let len = self.body.len() as i32;
        self.body.push((-1 * len, -1));
    }

    pub fn change_color(self: &mut Self, first_color: [f32; 4], second_color: [f32; 4]) {
        self.first_color = first_color;
        self.second_color = second_color;
    }
}

fn trans_color(first: [f32; 4], second: [f32; 4], t: f32) -> [f32; 4] {
    let mut result = [0.0; 4];
    for i in 0..4 {
        result[i] = first[i] * t + second[i] * (1.0 - t);
    }

    result
}
