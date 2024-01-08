use std::fmt::{self, Debug};
use std::str::FromStr;
use std::{
    convert::TryFrom,
    ops::{Index, IndexMut},
};

use crate::direction::Cardinal;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
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

impl Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Coordinate {
    pub fn new(row: isize, col: isize) -> Self {
        Self(row, col)
    }
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

    /// Returns the neighbour to the given direction
    pub fn neighbour(&self, direction: &Cardinal) -> Self {
        match direction {
            Cardinal::North => self.north(),
            Cardinal::South => self.south(),
            Cardinal::West => self.west(),
            Cardinal::East => self.east(),
        }
    }

    /// Returns the coordinate that is the given steps away in the given direction
    pub fn steps(&self, direction: &Cardinal, steps: usize) -> Self {
        let mut ret = *self;
        for _ in 0..steps {
            ret = ret.neighbour(direction);
        }
        ret
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

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = T::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| T::try_from(c))
                    .collect::<Result<Vec<T>, T::Error>>()
            })
            .collect::<Result<Vec<Vec<T>>, T::Error>>()?;

        Ok(grid.into())
    }
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

impl<T> fmt::Display for Grid<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.n {
            for j in 0..self.m {
                write!(f, "{}", self[(i, j).into()])?;
            }
            writeln!(f)?;
        }

        Ok(())
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

    pub fn get(&self, coord: Coordinate) -> Option<T> {
        if self.is_in_bounds(coord) {
            Some(self[coord])
        } else {
            None
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

    pub fn find_coordinate(&self, pred: impl Fn(&T) -> bool) -> Option<Coordinate> {
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
