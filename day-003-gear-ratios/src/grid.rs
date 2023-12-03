use std::{
    convert::TryFrom,
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
pub struct Coordinate(pub isize, pub isize);

impl From<(usize, usize)> for Coordinate {
    fn from(coords: (usize, usize)) -> Self {
        Coordinate(coords.0 as isize, coords.1 as isize)
    }
}

impl From<(isize, isize)> for Coordinate {
    fn from(coords: (isize, isize)) -> Self {
        Coordinate(coords.0, coords.1)
    }
}

impl Coordinate {
    pub fn x(&self) -> isize {
        self.1
    }

    pub fn y(&self) -> isize {
        -self.0
    }

    pub fn row(&self) -> isize {
        self.0
    }

    pub fn col(&self) -> isize {
        self.1
    }

    pub fn north(&self) -> Self {
        Self(self.0 - 1, self.1)
    }

    pub fn south(&self) -> Self {
        Self(self.0 + 1, self.1)
    }

    pub fn east(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    pub fn west(&self) -> Self {
        Self(self.0, self.1 - 1)
    }

    pub fn northeast(&self) -> Self {
        self.north().east()
    }

    pub fn northwest(&self) -> Self {
        self.north().west()
    }

    pub fn southeast(&self) -> Self {
        self.south().east()
    }

    pub fn southwest(&self) -> Self {
        self.south().west()
    }

    /// Returns the 4 cardinal neighbours: north, south, east, and west
    pub fn cardinal_neighbours(&self) -> [Self; 4] {
        [self.north(), self.south(), self.east(), self.west()]
    }

    /// Returns the 4 ordinal neighbours: northeast, northwest, southeast, southwest
    pub fn ordinal_neighbours(&self) -> [Self; 4] {
        [
            self.northeast(),
            self.northwest(),
            self.southeast(),
            self.southwest(),
        ]
    }

    /// Returns all 8 of the neighbours
    pub fn neighbours(&self) -> [Self; 8] {
        [
            self.north(),
            self.south(),
            self.east(),
            self.west(),
            self.northeast(),
            self.northwest(),
            self.southeast(),
            self.southwest(),
        ]
    }

    pub fn manhattan_distance(&self, other: &Self) -> usize {
        other.0.abs_diff(self.0) + other.1.abs_diff(self.1)
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub n: usize,
    pub m: usize,
}

impl<T> TryFrom<&[String]> for Grid<T>
where
    T: TryFrom<char>,
{
    type Error = T::Error;

    fn try_from(lines: &[String]) -> Result<Self, Self::Error> {
        let grid = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| T::try_from(c))
                    .collect::<Result<Vec<T>, T::Error>>()
            })
            .collect::<Result<Vec<Vec<T>>, T::Error>>()?;

        Ok(grid.into())
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Self {
        let n = grid.len();
        let m = grid[0].len();
        Self { grid, n, m }
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, idx: Coordinate) -> &Self::Output {
        &self.grid[idx.0 as usize][idx.1 as usize]
    }
}

impl<T> IndexMut<Coordinate> for Grid<T> {
    fn index_mut(&mut self, idx: Coordinate) -> &mut Self::Output {
        &mut self.grid[idx.0 as usize][idx.1 as usize]
    }
}

impl<T> Grid<T>
where
    T: Copy + PartialEq,
{
    pub fn new(n: usize, m: usize, default: T) -> Self {
        Self {
            grid: vec![vec![default; m]; n],
            n,
            m,
        }
    }

    pub fn is_in_bounds(&self, coord: Coordinate) -> bool {
        (0..self.n as isize).contains(&coord.0) && (0..self.m as isize).contains(&coord.1)
    }

    pub fn is_on_edge(&self, coord: Coordinate) -> bool {
        if self.is_in_bounds(coord) {
            let row = coord.0 as usize;
            let col = coord.1 as usize;
            row == 0 || row == self.n - 1 || col == 0 || col == self.m - 1
        } else {
            false
        }
    }

    pub fn find_index(&self, pred: impl Fn(&T) -> bool) -> Option<Coordinate> {
        for i in 0..self.n {
            for j in 0..self.m {
                if pred(&self.grid[i][j]) {
                    return Some(Coordinate(i as isize, j as isize));
                }
            }
        }

        None
    }
}
