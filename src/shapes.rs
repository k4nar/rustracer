use std::num::sqrt;

use color::{Color, Black, White, Red, Green, Blue};
use math3d::{Point, solve_poly};
use scene::Scene;

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

impl Shape {
  pub fn get_light(&self, scene: &Scene, inter: &Point, light: &Point) -> Color {
    let perp = self.shape.perp(inter);
    let norme_l = sqrt(light.x.pow(&2.) + light.y.pow(&2.) + light.z.pow(&2.));
    let norme_n = sqrt(perp.x.pow(&2.) + perp.y.pow(&2.) + perp.z.pow(&2.));
    let cos_a = (light.x * perp.x + light.y * perp.y + light.z * perp.z) / (norme_l * norme_n);

    if cos_a <= 0. {
      return Black;
    }

    Color {
      r: ((self.color.r as f64) * cos_a * (1. - self.shininess) + (scene.spot.color.r as f64) * cos_a * self.shininess) as u8,
      g: ((self.color.g as f64) * cos_a * (1. - self.shininess) + (scene.spot.color.g as f64) * cos_a * self.shininess) as u8,
      b: ((self.color.b as f64) * cos_a * (1. - self.shininess) + (scene.spot.color.b as f64) * cos_a * self.shininess) as u8
    }
  }
}

pub struct Sphere {
  radius: f64,
}

impl Drawable for Sphere {
  fn hit(&self, eye: &Point, vector: &Point) -> f64 {
    let a = vector.x.pow(&2.) + vector.y.pow(&2.) + vector.z.pow(&2.);
    let b = 2. * (eye.x * vector.x + eye.y * vector.y + eye.z * vector.z);
    let c = eye.x.pow(&2.) + eye.y.pow(&2.) + eye.z.pow(&2.) - self.radius.pow(&2.);
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
