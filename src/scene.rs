use std::f64::INFINITY;
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
    let mut min: f64 = INFINITY;
    let mut closest: Option<&'a Shape> = None;

    for obj in self.objects.iter() {
      let k = obj.shape.hit(&(self.eye - obj.pos), vector);
      if k > 0. && k < min {
        min = k;
        closest = Some(obj);
      }
    }

    return (closest, min);
  }

  fn get_shadow(&self, shape: &Shape, spot: &Spot, inter: &Point) -> bool {
    let limit = shape.shape.hit(inter, &spot.pos);

    for obj in self.objects.iter() {
      if obj as *Shape == shape as *Shape {
        continue;
      }

      let k = obj.shape.hit(inter, &spot.pos);
      if k > 0. && k < limit {
        return true;
      }
    }
    return false
    }

  pub fn get_color(&self, shape: &Shape, vector: &Point, dist: f64) -> Color {
    let (mut r, mut g, mut b) = (0., 0., 0.);

    let inter = self.eye + vector * dist;

    for spot in self.spots.iter() {
      if self.get_shadow(shape, spot, &inter) {
        continue;
      }

      let light = spot.pos - inter;
      let perp = shape.shape.perp(&inter);
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
