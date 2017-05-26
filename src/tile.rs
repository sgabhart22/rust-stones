pub enum BG_Colors {
    Pink,
    Blue,
    Green,
    Black
}

pub enum FG_Colors {
    Purple,
    Red,
    Yellow
}

pub enum Shapes {
    Diamond,
    Square,
    Circle,
    Triangle
}

pub struct Tile {
    bg: BG_Colors,
    fg: FG_Colors,
    shape: Shapes
}
