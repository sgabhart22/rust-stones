extern crate opengl_graphics;
extern crate piston_window;
extern crate piston;
extern crate graphics;

use piston_window::{OpenGL, PistonWindow, Size, WindowSettings, clear};
use opengl_graphics::GlGraphics;
use piston::input::*;
use graphics::{color, Transformed};

mod app;
mod board;
mod settings;

fn main() {
    let title = "Stones, ma dudes";
    let window_size = Size {
        height: 700,
        width: 800,
    };

    let opengl = OpenGL::V2_1;

    let mut window: PistonWindow =
        WindowSettings::new(title,
                            [window_size.width, window_size.height])
                            .opengl(opengl)
                            .exit_on_esc(true)
                            .build()
                            .unwrap();

    let mut app = app::App::new();
    let ref mut gl = GlGraphics::new(opengl);

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            app.on_render(&args, gl);
        }

        if let Some(button) = e.press_args() {
            app.on_button_press(&button);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }
    }
}
