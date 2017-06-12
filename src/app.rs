extern crate find_folder;

use opengl_graphics::{GlGraphics, Texture};
use piston::input::*;
use piston_window::*;
use graphics::{line, grid, color, Transformed, Rectangle, Image};
use std::path::Path;

use board;
use settings;

pub struct Vec2f {
    x: f64,
    y: f64
}

pub struct App {
    mouse_coords: Vec2f,
    board: board::Board,
    selected_cell: Option<Vec2f>
}

impl App {
    pub fn new() -> App {
        App {
            mouse_coords: Vec2f{ x: 0.0, y: 0.0 },
            board: board::Board::new(),
            selected_cell: None
        }
    }

    pub fn on_render(&mut self, args: &RenderArgs,
                     gl: &mut GlGraphics) {
        gl.draw(args.viewport(), |c, gl| {
        use graphics::*;

        clear([0.1255, 0.6980, 0.6667, 1.0], gl);
        let center = c.transform.trans(0.0, 0.0);

        let bg_image = Image::new().rect([0.0, 0.0, 800.0, 700.0]);
        let bg_texture = Texture::from_path(Path::new("assets/bg.png")).unwrap();

        bg_image.draw(&bg_texture, &c.draw_state, c.transform, gl);

        let line = line::Line::new(color::BLACK, 0.5);
        let _grid = grid::Grid{ cols: 10, rows: 10, units: settings::CELL_DIMS};

        _grid.draw(&line, &c.draw_state, center.trans(0.0, 0.0), gl);

        for x in 0..10 {
            for y in 0..10 {
                let ref current = self.board.get_cell(y, x);
                if current.pos == true {
                    let rect = Rectangle::new(current.cell_color.unwrap());
                    let dims = rectangle::square(x as f64 * settings::CELL_DIMS,
                                                 y as f64 * settings::CELL_DIMS,
                                                 settings::CELL_DIMS - 1.0);
                    rect.draw(dims, &c.draw_state, c.transform, gl);
                }
            }
        }
        });
    }

    pub fn on_button_press(&mut self, button: &Button) {
        match button {
            &Button::Keyboard(key) => {
                println!("Pressed key {:?}", key);
            },
            &Button::Mouse(button) => {
                self.on_mouse_click(&button);
            }
            &Button::Controller(_) => {}
        }
    }

    pub fn on_mouse_click(&mut self, button: &MouseButton) {
        println!("Pressed mouse button '{:?}'", button);
        println!("At coordinates {} {}", self.mouse_coords.x, self.mouse_coords.y);

        let ref point = Vec2f{ x: self.mouse_coords.x, y: self.mouse_coords.y };
        self.board.on_clicked_cell(point.x, point.y);
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
    }
}
