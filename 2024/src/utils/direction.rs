#[derive(Clone, Copy, Debug)]
pub enum Direction {
    // Orthogonal
    Up,
    Down,
    Left,
    Right,

    // Diagonal
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn orthogonal() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
    }

    pub fn diagonal() -> [Self; 4] {
        [Self::UpLeft, Self::UpRight, Self::DownLeft, Self::DownRight]
    }

    pub fn all() -> [Self; 8] {
        [
            Self::Up,
            Self::Down,
            Self::Left,
            Self::Right,
            Self::UpLeft,
            Self::UpRight,
            Self::DownLeft,
            Self::DownRight,
        ]
    }
}
