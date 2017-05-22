mod board;

struct Vec2f {
    x: f64,
    y: f64
}

struct App {
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
}
