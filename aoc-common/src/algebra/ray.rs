use num::Float;
use num::Num;

use crate::algebra::Point3;
use crate::algebra::Vector3;
use crate::algebra::EPSILON;
use crate::geometry::IntersectRay;
use crate::geometry::Intersection;

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

impl<T> Ray<T>
where
    T: Copy + Num + Into<f64>,
{
    pub fn to_f64(&self) -> Ray<f64> {
        let o = self.origin;
        let d = self.dir;

        Ray::new(
            Point3::new(o.x.into(), o.y.into(), o.z.into()),
            Vector3::new(d.x.into(), d.y.into(), d.z.into()),
        )
    }
}

impl IntersectRay for Ray<f64> {
    fn intersect(&self, ray: &Ray<f64>) -> Option<Intersection> {
        let (c, d) = (self.origin, ray.origin);
        let (e, f) = (self.dir, ray.dir);
        let g = d - c;
        let (h, k) = (f.cross(&g), f.cross(&e));
        let (hd, kd) = (h.magnitude(), k.magnitude());

        if hd < EPSILON || kd < EPSILON {
            return None;
        }

        let p = if h.dot(&k) > 0.0 {
            c + e * (hd / kd)
        } else {
            c - e * (hd / kd)
        };

        let t = ray.distance_to(p);
        let s = self.distance_to(p);

        if t > EPSILON && s > EPSILON {
            Some(Intersection::new(t, p))
        } else {
            None
        }
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

    #[test]
    fn intersection_test() {
        let a = Ray::new(Point3::new(6.0, 8.0, 4.0), Vector3::new(6.0, 7.0, 0.0));
        let b = Ray::new(Point3::new(6.0, 8.0, 2.0), Vector3::new(6.0, 7.0, 4.0));
        let expected = Intersection::new(50.5, Point3::new(9.0, 11.5, 4.0));
        assert!(a.intersect(&b).unwrap().approximately_equals(&expected));

        let a = Ray::new(Point3::new(19.0, 13.0, 0.0), Vector3::new(-2.0, 1.0, 0.0));
        let b = Ray::new(Point3::new(12.0, 31.0, 0.0), Vector3::new(-1.0, -2.0, 0.0));
        let expected = Intersection::new(29.0, Point3::new(6.2, 19.4, 0.0));
        assert!(a.intersect(&b).unwrap().approximately_equals(&expected));

        let a = Ray::new(Point3::new(18.0, 19.0, 0.0), Vector3::new(-1.0, -1.0, 0.0));
        let b = Ray::new(Point3::new(12.0, 31.0, 0.0), Vector3::new(-1.0, -2.0, 0.0));
        let expected = Intersection::new(90.0, Point3::new(-6.0, -5.0, 0.0));
        assert!(a.intersect(&b).unwrap().approximately_equals(&expected));

        let a = Ray::new(Point3::new(20.0, 19.0, 0.0), Vector3::new(1.0, -5.0, 0.0));
        let b = Ray::new(Point3::new(19.0, 13.0, 0.0), Vector3::new(-2.0, 1.0, 0.0));
        assert_eq!(a.intersect(&b), None);

        let a = Ray::new(Point3::new(19.0, 13.0, 0.0), Vector3::new(-2.0, 1.0, 0.0));
        let b = Ray::new(Point3::new(20.0, 19.0, 0.0), Vector3::new(1.0, -5.0, 0.0));
        assert_eq!(a.intersect(&b), None);
    }
}
