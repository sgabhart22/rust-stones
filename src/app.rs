extern crate piston;

use piston::input::*;

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

        let point = Vec2f{ x: self.mouse_coords.x, y: self.mouse_coords.y };
        let mut row = 0u8;
        let mut col = 0u8;
        let mut accumulated_height = 0.0f64;
        let mut accumulated_width = 0.0f64;

        if point.x > 650.0 || point.y > 650.0 {
            println!("Located outside grid");
        }
        else {
            for i in 1..10 {
                accumulated_width += settings::CELL_DIMS;
                if accumulated_width >= point.x { break };
                col += 1;
            }

            for j in 1..10 {
                accumulated_height += settings::CELL_DIMS;
                if accumulated_height >= point.y { break };
                row += 1;
            }

            println!("Located in cell {}, {}", row, col);
        }
    }

    pub fn on_mouse_move(&mut self, args: &[f64; 2]) {
        self.mouse_coords.x = args[0];
        self.mouse_coords.y = args[1];
    }
}
