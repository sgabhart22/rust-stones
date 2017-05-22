extern crate graphics;

use graphics::types::Color;

pub struct Coords {
    pub x: u8,
    pub y: u8
}

#[derive(Copy, Clone)]
pub struct Cell {
    cell_color: Option<Color>
}

pub struct Board {
    pub cells: [[Cell; 10]; 10]
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board{ cells: [[Cell{ cell_color: None }; 10]; 10]};
        board
    }
}
