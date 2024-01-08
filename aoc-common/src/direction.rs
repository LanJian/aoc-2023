#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Cardinal {
    North,
    East,
    South,
    West,
}

impl Cardinal {
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

    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }

    pub fn right(&self) -> Self {
        self.left().opposite()
    }
}
