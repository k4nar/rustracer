use std::num::{powf, sqrt};

use color::{Color, Black};
use point::Point;

trait Shape {
  fn hit(&self, eye: &Point, vector: &Point) -> f64;
  fn perp(&self, inter: &Point) -> Point;
}

pub struct Object {
  pos: Point,
  shininess: f64,
  color: Color,
  shape: ~Shape
}

pub struct Sphere {
  radius: f64,
}

impl Shape for Sphere {
  fn hit(&self, origin: &Point, vector: &Point) -> f64 {
    let a = vector.scalar_product(vector);
    let b = 2. * origin.scalar_product(vector);
    let c = origin.scalar_product(origin) - powf(self.radius, 2.);
    return solve_poly(a, b, c);
  }

  fn perp(&self, inter: &Point) -> Point {
    Point::new(inter.x, inter.y, inter.z)
  }
}

pub struct Plane;

impl Shape for Plane {
  fn hit(&self, eye: &Point, vector: &Point) -> f64 {
    -eye.z / vector.z
  }

  fn perp(&self, inter: &Point) -> Point {
    Point::new(0., 0., 100.)
  }
}

pub fn solve_poly(a: f64, b: f64, c: f64) -> f64 {
  let delta = powf(b,  2.0) - 4. * a * c;

  if delta < 0. {
    return 0.;
  }

  let sign = match c {
    _ if c < 0. => 1.,
    _ => -1.
  };

  (-b + sign * sqrt(delta)) / (2. * a)
}

