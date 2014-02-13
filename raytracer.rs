extern mod png;

use std::num::{sqrt, min, max, cos, sin};

static WIDTH: int = 800;
static HEIGHT: int = 800;

struct Color {
  r: u8,
  g: u8,
  b: u8
}

static Red: Color = Color { r: 255, g: 0, b: 0 };
static Green: Color = Color { r: 0, g: 255, b: 0 };
static Blue: Color = Color { r: 0, g: 0, b: 255 };
static White: Color = Color { r: 255, g: 255, b: 255 };
static Black: Color = Color { r: 0, g: 0, b: 0 };

struct Point {
  x: f64,
  y: f64,
  z: f64
}

struct Spot {
  pos: Point,
  color: Color,
}

struct Scene {
  eye: Point,
  spot: Spot,
  objects: ~[Shape]
}

fn solve_poly(a: f64, b: f64, c: f64) -> f64 {
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

trait Drawable {
  fn hit(&self, eye: &Point, vector: &Point) -> f64;
  fn perp(&self, inter: &Point) -> ~Point;
}

struct Shape {
  pos: Point,
  shininess: f64,
  color: Color,
  shape: ~Drawable
}

impl Shape {
  fn get_light(&self, spot: &Spot, inter: &Point, light: &Point) -> Color {
    let perp = self.shape.perp(inter);
    let norme_l = sqrt(light.x.pow(&2.) + light.y.pow(&2.) + light.z.pow(&2.));
    let norme_n = sqrt(perp.x.pow(&2.) + perp.y.pow(&2.) + perp.z.pow(&2.));
    let cos_a = (light.x * perp.x + light.y * perp.y + light.z * perp.z) / (norme_l * norme_n);

    if cos_a <= 0. {
      return Black;
    }

    Color {
      r: ((self.color.r as f64) * cos_a * (1. - self.shininess) + (spot.color.r as f64) * cos_a * self.shininess) as u8,
      g: ((self.color.g as f64) * cos_a * (1. - self.shininess) + (spot.color.g as f64) * cos_a * self.shininess) as u8,
      b: ((self.color.b as f64) * cos_a * (1. - self.shininess) + (spot.color.b as f64) * cos_a * self.shininess) as u8
    }
  }
}

struct Sphere {
  radius: f64,
}

impl Drawable for Sphere {
  fn hit(&self, eye: &Point, vector: &Point) -> f64 {
    let a = vector.x.pow(&2.) + vector.y.pow(&2.) + vector.z.pow(&2.);
    let b = 2. * (eye.x * vector.x + eye.y * vector.y + eye.z * vector.z);
    let c = eye.x.pow(&2.) + eye.y.pow(&2.) + eye.z.pow(&2.) - self.radius.pow(&2.);
    return solve_poly(a, b, c);
  }

  fn perp(&self, inter: &Point) -> ~Point {
    ~Point { x: inter.x, y: inter.y, z: inter.z }
  }
}

struct Plane;

impl Drawable for Plane {
  fn hit(&self, eye: &Point, vector: &Point) -> f64 {
    match -eye.z / vector.z {
      k if k > 0. => k,
      _ => 0.
    }
  }

  fn perp(&self, inter: &Point) -> ~Point {
    ~Point { x: 0., y: 0., z: 100. }
  }
}

impl Scene {
  fn get_closest<'a>(&'a self, vector: &'a Point) -> (Option<&'a Shape>, f64) {
    let mut min: f64 = 0.;
    let mut closest: Option<&'a Shape> = None;

    for obj in self.objects.iter() {
      let e = Point { x: self.eye.x - obj.pos.x, y: self.eye.y - obj.pos.y, z: self.eye.z - obj.pos.z };
      let v = Point { x: vector.x, y: vector.y, z: vector.z };
      let k = obj.shape.hit(&e, &v);
      if k != 0. && (min == 0. || k < min) {
        min = k;
        closest = Some(obj);
      }
    }

    return (closest, min);
  }
}

fn main() {
  let mut pixels = ~[Black, ..((WIDTH * HEIGHT) as uint)];

  let scene = ~Scene {
    eye: Point { x: -300., y: 0., z: 200. },
    spot: Spot {
      pos: Point { x: -300., y: 100., z: 200. },
      color: White
    },
    objects: ~[
      // Shape {
      //   pos: Point { x: 0., y: 0., z: 100. },
      //   shininess: 0.2,
      //   color: Red,
      //   shape: ~Sphere { radius: 160. }
      // },
      Shape {
        pos: Point { x: 0., y: 0., z: 0.},
        shininess: 0.1,
        color: Green,
        shape: ~Plane
      },
    ]
  };

  for x in range(0, WIDTH) {
    for y in range(0, HEIGHT) {
      let vector = ~Point {
        x: 100.,
        y: (WIDTH / 2 - x) as f64,
        z: (HEIGHT / 2 - y) as f64
      };

      let (obj, k) = scene.get_closest(vector);

      if obj.is_none() {
        pixels[y * WIDTH + x] = Black;
      }
      else {
        let closest = obj.unwrap();

        // // Shadow
        // let inter = Point {
        //   x: eye.x + closest * vector.x,
        //   y: eye.y + closest * vector.y,
        //   z: eye.z + closest * vector.z
        // };
        // let light = Point {
        //   x: spot.pos.x - inter.x,
        //   y: spot.pos.y - inter.y,
        //   z: spot.pos.z - inter.z
        // };

        let inter = ~Point {
          x: (scene.eye.x - closest.pos.x) + k * vector.x,
          y: (scene.eye.y - closest.pos.y) + k * vector.y,
          z: (scene.eye.z - closest.pos.z) + k * vector.z
        };
        let light = ~Point {
          x: scene.spot.pos.x - inter.x,
          y: scene.spot.pos.y - inter.y,
          z: scene.spot.pos.z - inter.z
        };

        pixels[y * WIDTH + x] = match k {
          0. => Black,
          _ => closest.get_light(&scene.spot, inter, light)
        }
      }
    }
  }

  let img = png::Image {
    width: WIDTH as u32,
    height: HEIGHT as u32,
    color_type: png::RGB8,
    pixels: pixels.flat_map(|&Color { r, g, b }| { ~[r, g, b] })
  };

  png::store_png(&img, &Path::new("out.png"));
}
