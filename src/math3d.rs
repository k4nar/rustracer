use std::num::sqrt;

pub struct Point {
  x: f64,
  y: f64,
  z: f64
}

impl Point {
  pub fn new(x: f64, y: f64, z: f64) -> Point {
    Point { x: x, y: y, z: z }
  }
}

pub fn solve_poly(a: f64, b: f64, c: f64) -> f64 {
  let delta = b.pow(&2.0) - 4. * a * c;

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
