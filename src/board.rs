extern crate graphics;

use graphics::types::Color;
use graphics::color;

use settings;

pub struct Coords {
    pub x: u8,
    pub y: u8
}

pub struct Vec2f {
    x: f64,
    y: f64
}

#[derive(Copy, Clone)]
pub struct Cell {
    cell_color: Option<Color>,
    pos: bool
}

#[derive(Copy, Clone)]
pub struct Board {
    pub cells: [[Cell; 10]; 10]
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board{ cells: [[Cell{ cell_color: None, pos: false }; 10]; 10]};
        board
    }

    pub fn on_clicked_cell(&mut self, x_comp: f64, y_comp: f64) {
        let mut point = Vec2f{ x: x_comp, y: y_comp };
        let mut row = 0u8;
        let mut col = 0u8;
        let mut accumulated_height = 0.0f64;
        let mut accumulated_width = 0.0f64;

        if point.x > 650.0 || point.y > 650.0 {
            println!("Located outside grid");
            return
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

            let mut target: Cell = self.cells[row as usize][col as usize];
        }

        fn toggle_cell(c: Cell) {

        }
    }
}
