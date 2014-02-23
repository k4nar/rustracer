use std::num::{powf, sqrt};

pub struct Point {
  x: f64,
  y: f64,
  z: f64
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x: x, y: y, z: z }
  }

  pub fn norm(&self) -> f64 {
    sqrt(powf(self.x, 2.) + powf(self.y, 2.) + powf(self.z, 2.))
  }

  pub fn normalize(&self) -> Point {
    let norm = self.norm();
    Point {x: self.x / norm, y: self.y / norm, z: self.z / norm}
  }

  pub fn scalar_product(&self, other: &Point) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Add<Point, Point> for Point {
  fn add(&self, other: &Point) -> Point {
    Point {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}

impl Sub<Point, Point> for Point {
  fn sub(&self, other: &Point) -> Point {
    Point {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

trait MulRhs<T> {
  fn mul(&self, lhs: &Point) -> T;
}

impl<R, L: MulRhs<R>> Mul<L, R> for Point {
  fn mul(&self, other: &L) -> R {
    other.mul(self)
  }
}

impl MulRhs<Point> for Point {
  fn mul(&self, other: &Point) -> Point {
    Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
  }
}

impl MulRhs<Point> for f64 {
  fn mul(&self, other: &Point) -> Point {
    Point {x: *self * other.x, y: *self * other.y, z: *self * other.z}
  }
}
