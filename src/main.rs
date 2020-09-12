extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

mod food;
mod snake;
mod snake_app;

use snake_app::*;

const WINDOW_SIZE: [u32; 2] = [1000, 600];

fn main() {
    let opengl = OpenGL::V4_5;
    let mut window: GlutinWindow = WindowSettings::new("Snake", WINDOW_SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = SnakeApp::new(opengl)
        .change_size(50)
        .food_color([1.0, 0.0, 1.0, 1.0])
        .update_delta(0.05);

    let mut event = Events::new(EventSettings::new());

    let mut button: Option<Button> = None;
    while let Some(e) = event.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(args);
        }
        if let Some(args) = e.update_args() {
            if app.update(args, &mut button) == -2 {
                break;
            }
        }
        if let Some(args) = e.press_args() {
            button = Some(args);
        }
    }
}
