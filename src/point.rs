use std::ops::{Mul, Add, Sub};

#[derive(Copy, Clone)]
pub struct Point {
  pub x: f64,
  pub y: f64,
  pub z: f64
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x: x, y: y, z: z }
  }

  pub fn norm(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
  }

  pub fn normalize(&self) -> Point {
    let norm = self.norm();
    Point {x: self.x / norm, y: self.y / norm, z: self.z / norm}
  }

  pub fn scalar_product(&self, other: Point) -> f64 {
    self.x * other.x + self.y * other.y + self.z * other.z
  }
}

impl Add<Point> for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
  }
}


impl Sub<Point> for Point {
  type Output = Point;

  fn sub(self, other: Point) -> Point {
    Point {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
  }
}

impl Mul<f64> for Point {
  type Output = Point;

  fn mul(self, other: f64) -> Point {
    other.mul(self)
  }
}

impl Mul<Point> for Point {
  type Output = Point;

  fn mul(self, other: Point) -> Point {
    Point {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
  }
}

impl Mul<Point> for f64 {
  type Output = Point;

  fn mul(self, other: Point) -> Point {
    Point {x: self * other.x, y: self * other.y, z: self * other.z}
  }
}
