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
}
