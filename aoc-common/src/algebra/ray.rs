use num::Float;
use num::Num;

use crate::algebra::Point3;
use crate::algebra::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray<T>
where
    T: Copy + Num,
{
    pub origin: Point3<T>,
    pub dir: Vector3<T>,
}

impl<T> Ray<T>
where
    T: Copy + Num,
{
    pub fn new(origin: Point3<T>, dir: Vector3<T>) -> Self {
        Self { origin, dir }
    }

    pub fn distance_to(&self, point: Point3<T>) -> T {
        (point - self.origin).dot(&self.dir)
    }
}

impl<T> Ray<T>
where
    T: Copy + Num + Float,
{
    pub fn normalize(&self) -> Self {
        Ray::new(self.origin, self.dir.normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to() {
        assert_eq!(
            Ray::new(Point3::origin(), Vector3::i()).distance_to(Point3::new(4.0, 0.0, 0.0)),
            4.0
        );
    }
}
