extern crate opengl_graphics;
extern crate piston_window;
extern crate piston;
extern crate graphics;

use piston_window::{OpenGL, PistonWindow, Size, WindowSettings, clear};
use opengl_graphics::GlGraphics;
use piston::input::*;
use graphics::{line, grid, color, Transformed};

struct Vec2f {
    x: f64,
    y: f64
}

fn main() {
    let title = "First stones run";
    let window_size = Size {
        height: 700,
        width: 800,
    };

    let mut mouse_coords = Vec2f{ x: 0.0, y: 0.0 };

    let opengl = OpenGL::V2_1;

    let mut window: PistonWindow =
        WindowSettings::new(title,
                            [window_size.width, window_size.height])
                            .opengl(opengl)
                            .exit_on_esc(true)
                            .build()
                            .unwrap();

    let line = line::Line::new(color::BLACK, 0.5);
    let _grid = grid::Grid{ cols: 10, rows: 10, units: 65.0};

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.7214, 0.0, 0.6667, 0.8], g);
            let center = c.transform.trans(0.0, 0.0);

            _grid.draw(&line, &c.draw_state, center.trans(0.0, 0.0), g);
        });

        e.mouse_cursor(|x, y| {
            let cursor = [x, y];
            println!("Mouse moved '{} {}'", cursor[0], cursor[1]);
        });

        if let Some(Button::Mouse(button)) = e.press_args() {
            println!("Pressed mouse button '{:?}'", button);
            println!("At coordinates {} {}", mouse_coords.x, mouse_coords.y);

            let point = Vec2f{ x:mouse_coords.x, y:mouse_coords.y };
            let mut row = 0u8;
            let mut col = 0u8;
            let mut accumulated_height = 0.0f64;
            let mut accumulated_width = 0.0f64;

            for i in 1..10 {
                accumulated_width += 65.0;
                if accumulated_width >= point.x { break };
                col += 1;
            }

            for j in 1..10 {
                accumulated_height += 65.0;
                if accumulated_height >= point.y { break };
                row += 1;
            }

            println!("Located in cell {}, {}", row, col);
        }

        if let Some(args) = e.mouse_cursor_args() {
            mouse_coords.x = args[0];
            mouse_coords.y = args[1];
        }
    }
}
