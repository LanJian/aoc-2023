#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl CardinalDirection {
    pub fn all() -> [Self; 4] {
        [Self::North, Self::South, Self::West, Self::East]
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}
