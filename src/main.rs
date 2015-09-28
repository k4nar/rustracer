extern crate image;

use std::fs::File;
use std::path::Path;

pub mod color;
pub mod point;
pub mod scene;
pub mod shapes;

use color::{Color, Black, White, Red, Green, Blue};
use point::Point;
use scene::{Scene, Spot};
use shapes::{Object, Sphere, Plane};

static WIDTH: u32 = 1000;
static HEIGHT: u32 = 1000;

fn main() {
  let scene = Scene {
    eye: Point::new(-12000., 0., 0.),
    spots: &[
      Spot {
        pos: Point::new(-10000., -500., 5000.),
        color: White.clone()
      },
    ],
    objects: &[
      Object {
        pos: Point::new(0., 100., 0.),
        shininess: 0.,
        reflection: 0.1,
        color: White.clone(),
        shape: &Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., -100., 0.),
        shininess: 0.,
        reflection: 1.,
        color: White.clone(),
        shape: &Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., 0., -100.),
        shininess: 0.,
        reflection: 0.3,
        color: White.clone(),
        shape: &Plane
      },
    ]
  };

  let mut img = image::ImageBuffer::new(WIDTH, HEIGHT);

  for x in (0..WIDTH) {
    for y in (0..HEIGHT) {
      let vector = Point {
        x: 10000.,
        y: WIDTH as f64 / 2. - x as f64,
        z: HEIGHT as f64 / 2. - y as f64
      }.normalize();

      let color = match scene.get_closest(scene.eye, vector, None) {
        (Some(obj), k) if k > 0. => scene.get_color(obj, scene.eye, vector, k, 5),
        _ => Black.clone()
      };

      img.put_pixel(x, y, image::Rgb([color.r, color.g, color.b]));
    }
  }

  let ref mut fout = File::create(&Path::new("out.png")).unwrap();
  let _ = image::ImageRgb8(img).save(fout, image::PNG);
}
