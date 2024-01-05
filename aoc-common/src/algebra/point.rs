use std::ops::{Add, Div, Mul, Neg, Sub};

use num::Num;

use super::Vector3;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point2<T>
where
    T: Copy + Num,
{
    pub x: T,
    pub y: T,
}

impl<T> Point2<T>
where
    T: Copy + Num,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<[T; 2]> for Point2<T>
where
    T: Copy + Num,
{
    fn from(p: [T; 2]) -> Self {
        Self::new(p[0], p[1])
    }
}

impl<T> From<(T, T)> for Point2<T>
where
    T: Copy + Num,
{
    fn from(p: (T, T)) -> Self {
        Self::new(p.0, p.1)
    }
}

impl<T> From<Point3<T>> for Point2<T>
where
    T: Copy + Num,
{
    fn from(p: Point3<T>) -> Self {
        Self::new(p.x, p.y)
    }
}

impl<T> Div<T> for Point2<T>
where
    T: Copy + Num + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Point3<T>
where
    T: Copy + Num,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T>
where
    T: Copy + Num,
{
    pub fn origin() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Add<Vector3<T>> for Point3<T>
where
    T: Copy + Num + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Point3<T>
where
    T: Copy + Num + Sub,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Sub<Vector3<T>> for Point3<T>
where
    T: Copy + Num,
{
    type Output = Point3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Neg for Point3<T>
where
    T: Copy + Num + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<T> Mul<T> for Point3<T>
where
    T: Copy + Num + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add_vector() {
        let expected = Point3::new(4.0, 3.0, 1.0);
        let actual = Point3::new(1.0, 1.0, 0.0) + Vector3::new(3.0, 2.0, 1.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn point_subtract_point() {
        let expected = Vector3::new(3.0, 4.0, -3.0);
        let actual = Point3::new(5.0, 5.0, 0.0) - Point3::new(2.0, 1.0, 3.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn negate() {
        let expected = Point3::new(-1.0, 0.0, 0.0);
        let actual = -Point3::new(1.0, 0.0, 0.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn point_times_scalar() {
        let expected = Point3::new(0.0, 6.0, 0.0);
        let actual = Point3::new(0.0, 1.0, 0.0) * 6.0;
        assert_eq!(actual, expected);
    }
}
