use crate::algebra::{Point3, Ray, EPSILON};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub position: Point3<f64>,
}

impl Intersection {
    pub fn new(t: f64, position: Point3<f64>) -> Self {
        Self { t, position }
    }

    pub fn approximately_equals(&self, other: &Self) -> bool {
        (self.t - other.t).abs() < EPSILON
            && (self.position.x - other.position.x).abs() < EPSILON
            && (self.position.y - other.position.y).abs() < EPSILON
            && (self.position.z - other.position.z).abs() < EPSILON
    }
}

pub trait IntersectRay {
    fn intersect(&self, ray: &Ray<f64>) -> Option<Intersection>;
}
