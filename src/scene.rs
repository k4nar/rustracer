use std::f64::INFINITY;
use std::cmp::min;

use shapes::Object;
use point::Point;
use color::{Color, Black};

pub struct Spot {
  pos: Point,
  color: Color,
}

pub struct Scene {
  eye: Point,
  spots: ~[Spot],
  objects: ~[Object]
}

impl Scene {
  pub fn get_closest<'a>(&'a self, vector: &'a Point) -> (Option<&'a Object>, f64) {
    let mut min: f64 = INFINITY;
    let mut closest: Option<&'a Object> = None;

    for obj in self.objects.iter() {
      let k = obj.shape.hit(&(self.eye - obj.pos), vector);
      if k > 0. && k < min {
        min = k;
        closest = Some(obj);
      }
    }

    return (closest, min);
  }

  fn get_shadow(&self, cur: &Object, light: &Point, inter: &Point) -> bool {
    for obj in self.objects.iter() {
      if obj as *Object == cur as *Object {
        continue;
      }

      let k = obj.shape.hit(&(inter - obj.pos), light);
      if k > 0. && k < 1. {
        return true;
      }
    }

    false
  }

  pub fn get_color(&self, obj: &Object, vector: &Point, dist: f64) -> Color {
    let (mut r, mut g, mut b) = (0., 0., 0.);

    let inter = self.eye + vector * dist;

    for spot in self.spots.iter() {
      let light = spot.pos - inter;

      if self.get_shadow(obj, &light, &inter) {
        continue;
      }

      let perp = obj.shape.perp(&inter);
      let cos_a = perp.normalize().scalar_product(&light.normalize());

      if cos_a <= 0. || cos_a.is_nan() {
        continue;
      }

      r += (obj.color.r as f64) * cos_a * (1. - obj.shininess) + (spot.color.r as f64) * cos_a * obj.shininess;
      g += (obj.color.g as f64) * cos_a * (1. - obj.shininess) + (spot.color.g as f64) * cos_a * obj.shininess;
      b += (obj.color.b as f64) * cos_a * (1. - obj.shininess) + (spot.color.b as f64) * cos_a * obj.shininess;
    }

    Color {
      r: (min(r, 255.) as u8),
      g: (min(g, 255.) as u8),
      b: (min(b, 255.) as u8)
    }
  }
}
