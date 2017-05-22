extern crate opengl_graphics;
extern crate piston_window;
extern crate piston;
extern crate graphics;

use piston_window::{OpenGL, PistonWindow, Size, WindowSettings, clear};
use opengl_graphics::GlGraphics;
use piston::input::*;
use graphics::{line, grid, color, Transformed};

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

    let mut app = app::App::new();
    let mut window: PistonWindow =
        WindowSettings::new(title,
                            [window_size.width, window_size.height])
                            .opengl(opengl)
                            .exit_on_esc(true)
                            .build()
                            .unwrap();

    let line = line::Line::new(color::BLACK, 0.5);
    let _grid = grid::Grid{ cols: 10, rows: 10, units: settings::CELL_DIMS};

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.1255, 0.6980, 0.6667, 0.7], g);
            let center = c.transform.trans(0.0, 0.0);

            _grid.draw(&line, &c.draw_state, center.trans(0.0, 0.0), g);
        });

        if let Some(button) = e.press_args() {
            app.on_button_press(&button);
        }

        if let Some(args) = e.mouse_cursor_args() {
            app.on_mouse_move(&args);
        }
    }
}
