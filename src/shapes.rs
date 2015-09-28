use color::Color;
use point::Point;

pub trait Shape {
  fn hit(&self, eye: Point, vector: Point) -> f64;
  fn perp(&self, inter: Point) -> Point;
}

pub struct Object<'a> {
  pub pos: Point,
  pub color: Color,
  pub shape: &'a Shape,
  pub shininess: f64,
  pub reflection: f64,
}

pub struct Sphere {
  pub radius: f64,
}

impl Shape for Sphere {
  fn hit(&self, origin: Point, vector: Point) -> f64 {
    let a = vector.scalar_product(vector);
    let b = 2. * origin.scalar_product(vector);
    let c = origin.scalar_product(origin) - self.radius.powi(2);
    return solve_poly(a, b, c);
  }

  fn perp(&self, inter: Point) -> Point {
    inter
  }
}

pub struct Plane;

impl Shape for Plane {
  fn hit(&self, eye: Point, vector: Point) -> f64 {
    -eye.z / vector.z
  }

  fn perp(&self, inter: Point) -> Point {
    Point::new(0., 0., 100.)
  }
}

fn solve_poly(a: f64, b: f64, c: f64) -> f64 {
  let delta = b.powi(2) - 4. * a * c;

  if delta < 0. {
    return 0.;
  }

  let sign = match c {
    _ if c < 0. => 1.,
    _ => -1.
  };

  (-b + sign * delta.sqrt()) / (2. * a)
}
