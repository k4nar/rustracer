use shapes::Shape;
use math3d::Point;
use color::Color;

pub struct Spot {
  pos: Point,
  color: Color,
}

pub struct Scene {
  eye: Point,
  spot: Spot,
  objects: ~[Shape]
}

impl Scene {
  pub fn get_closest<'a>(&'a self, vector: &'a Point) -> (Option<&'a Shape>, f64) {
    let mut min: f64 = 0.;
    let mut closest: Option<&'a Shape> = None;

    for obj in self.objects.iter() {
      let e = Point::new(self.eye.x - obj.pos.x, self.eye.y - obj.pos.y, self.eye.z - obj.pos.z);
      let v = Point::new(vector.x, vector.y, vector.z);
      let k = obj.shape.hit(&e, &v);
      if k != 0. && (min == 0. || k < min) {
        min = k;
        closest = Some(obj);
      }
    }

    return (closest, min);
  }
}
