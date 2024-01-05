use std::ops::{Add, Div, Mul, Neg, Sub};

use num::{Float, Num};

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T>
where
    T: Copy + Num,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Copy + Num,
{
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn i() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    pub fn j() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
            z: T::zero(),
        }
    }

    pub fn k() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::one(),
        }
    }

    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, rhs: &Self) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let Self {
            x: ref bx,
            y: ref by,
            z: ref bz,
        } = self;
        let Self {
            x: ref cx,
            y: ref cy,
            z: ref cz,
        } = rhs;

        Self::new(
            *by * *cz - *bz * *cy,
            *bz * *cx - *bx * *cz,
            *bx * *cy - *by * *cx,
        )
    }

    pub fn norm(&self) -> T {
        self.dot(self)
    }
}

impl<T> Vector3<T>
where
    T: Copy + Num + Float,
{
    pub fn magnitude(&self) -> T {
        self.norm().sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }
}

impl<T> Add for Vector3<T>
where
    T: Copy + Num + Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> Sub for Vector3<T>
where
    T: Copy + Num + Sub<Output = T>,
{
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Neg for Vector3<T>
where
    T: Copy + Num + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::zero() - self
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Copy + Num + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Copy + Num + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::Vector3;

    #[test]
    fn add() {
        let expected = Vector3::new(1.0, 1.0, 0.0);
        let actual = Vector3::i() + Vector3::j();
        assert_eq!(actual, expected);
    }

    #[test]
    fn subtract() {
        let expected = Vector3::new(1.0, -1.0, 0.0);
        let actual = Vector3::i() - Vector3::j();
        assert_eq!(actual, expected);
    }

    #[test]
    fn negate() {
        let expected = Vector3::new(-1.0, 0.0, 0.0);
        let actual = -Vector3::i();
        assert_eq!(actual, expected);
    }

    #[test]
    fn vector_times_scalar() {
        let expected = Vector3::new(0.0, 6.0, 0.0);
        let actual = Vector3::j() * 6.0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn vector_div_scalar() {
        assert_eq!(
            Vector3::new(4.0, 10.0, 0.0) / 2.0,
            Vector3::new(2.0, 5.0, 0.0)
        );
    }

    #[test]
    fn div_by_zero() {
        assert_eq!(
            Vector3::new(4.0, 10.0, 1.0) / 0.0,
            Vector3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        );
    }
}
