use std::f64::INFINITY;

use shapes::Object;
use point::Point;
use color::{Color, Black};

pub struct Spot {
  pub pos: Point,
  pub color: Color,
}

pub struct Scene<'a> {
  pub eye: Point,
  pub spots: &'a [Spot],
  pub objects: &'a [Object<'a>]
}

impl<'a> Scene<'a> {
  pub fn get_closest(&'a self, origin: Point, vector: Point, cur: Option<&Object>) -> (Option<&'a Object>, f64) {
    let mut min: f64 = INFINITY;
    let mut closest: Option<&'a Object> = None;

    for obj in self.objects.iter() {
      match cur {
        Some(o) if (obj as *const Object) == (o as *const Object) => continue,
        _ => ()
      }

      let k = obj.shape.hit(origin - obj.pos, vector);
      if k > 0. && k < min {
        min = k;
        closest = Some(obj);
      }
    }

    return (closest, min);
  }

  fn get_shadow(&'a self, cur: &Object, inter: Point, light: Point) -> bool {
    for obj in self.objects.iter() {
      if (obj as *const Object) == (cur as *const Object) {
        continue;
      }

      let k = obj.shape.hit(inter - obj.pos, light);
      if k > 0. && k < 1. {
        return true;
      }
    }

    false
  }

  pub fn get_color(&'a self, obj: &'a Object, origin: Point, vector: Point, dist: f64, count: u32) -> Color {
    let (mut r, mut g, mut b) = (0., 0., 0.);

    let inter = origin + vector * dist;
    let inter_simple = (origin - obj.pos) + vector * dist;
    let perp = obj.shape.perp(inter_simple).normalize();

    for spot in self.spots.iter() {
      let light = spot.pos - inter;

      if self.get_shadow(obj, inter, light) {
        continue;
      }

      let cos_a = perp.scalar_product(light.normalize());

      if cos_a <= 0. || cos_a.is_nan() {
        continue;
      }

      r += (obj.color.r as f64) * cos_a * (1. - obj.shininess) + (spot.color.r as f64) * cos_a * obj.shininess;
      g += (obj.color.g as f64) * cos_a * (1. - obj.shininess) + (spot.color.g as f64) * cos_a * obj.shininess;
      b += (obj.color.b as f64) * cos_a * (1. - obj.shininess) + (spot.color.b as f64) * cos_a * obj.shininess;
    }

    if obj.reflection > 0. && count > 0 {
      let reflection = perp * -2. * perp.scalar_product(vector) + vector;
      let refc = match self.get_closest(inter, reflection, Some(obj)) {
        (Some(o), k) if k > 0. => self.get_color(o, inter, reflection, k, count - 1),
        _ => Black.clone()
      };
      r = r * (1. - obj.reflection) + (refc.r as f64) * obj.reflection;
      g = g * (1. - obj.reflection) + (refc.g as f64) * obj.reflection;
      b = b * (1. - obj.reflection) + (refc.b as f64) * obj.reflection;
    }

    Color {
      r: r.min(255.) as u8,
      g: g.min(255.) as u8,
      b: b.min(255.) as u8
    }
  }
}
