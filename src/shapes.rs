use std::num::{powf, sqrt};

use color::{Color, Black};
use point::Point;

trait Drawable {
  fn hit(&self, eye: &Point, vector: &Point) -> f64;
  fn perp(&self, inter: &Point) -> Point;
}

pub struct Shape {
  pos: Point,
  shininess: f64,
  color: Color,
  shape: ~Drawable
}

pub struct Sphere {
  radius: f64,
}

impl Drawable for Sphere {
  fn hit(&self, eye: &Point, vector: &Point) -> f64 {
    let a = vector.scalar_product(vector);
    let b = 2. * eye.scalar_product(vector);
    let c = eye.scalar_product(eye) - powf(self.radius, 2.);
    return solve_poly(a, b, c);
  }

  fn perp(&self, inter: &Point) -> Point {
    Point::new(inter.x, inter.y, inter.z)
  }
}

pub struct Plane;

impl Drawable for Plane {
  fn hit(&self, eye: &Point, vector: &Point) -> f64 {
    match -eye.z / vector.z {
      k if k > 0. => k,
      _ => 0.
    }
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
  let k = (-b + sign * sqrt(delta)) / (2. * a);
  return match k {
    _ if k > 0. => k,
    _ => 0.
  }
}

