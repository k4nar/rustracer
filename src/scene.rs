use std::cmp::min;

use shapes::Shape;
use point::Point;
use color::{Color, Black};

pub struct Spot {
  pos: Point,
  color: Color,
}

pub struct Scene {
  eye: Point,
  spots: ~[Spot],
  objects: ~[Shape]
}

impl Scene {
  pub fn get_closest<'a>(&'a self, vector: &'a Point) -> (Option<&'a Shape>, f64) {
    let mut min: f64 = 0.;
    let mut closest: Option<&'a Shape> = None;

    for obj in self.objects.iter() {
      let k = obj.shape.hit(&(self.eye - obj.pos), vector);
      if k != 0. && (min == 0. || k < min) {
        min = k;
        closest = Some(obj);
      }
    }

    return (closest, min);
  }

  pub fn get_color(&self, shape: &Shape, inter: &Point) -> Color {
    let (mut r, mut g, mut b) = (0., 0., 0.);

    for spot in self.spots.iter() {
      let light = spot.pos - *inter;
      let perp = shape.shape.perp(inter);
      let cos_a = perp.normalize().scalar_product(&light.normalize());

      if cos_a <= 0. {
        continue;
      }

      r += (shape.color.r as f64) * cos_a * (1. - shape.shininess) + (spot.color.r as f64) * cos_a * shape.shininess;
      g += (shape.color.g as f64) * cos_a * (1. - shape.shininess) + (spot.color.g as f64) * cos_a * shape.shininess;
      b += (shape.color.b as f64) * cos_a * (1. - shape.shininess) + (spot.color.b as f64) * cos_a * shape.shininess;
    }

    Color {
      r: (min(r, 255.) as u8),
      g: (min(g, 255.) as u8),
      b: (min(b, 255.) as u8)
    }
  }
}
