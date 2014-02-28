use png;
use color::{Color, Black, White, Red, Green, Blue};
use point::Point;
use scene::{Scene, Spot};
use shapes::{Object, Sphere, Plane};

static WIDTH: int = 1000;
static HEIGHT: int = 1000;

#[main]
fn main() {
  let mut pixels = ~[Black, ..((WIDTH * HEIGHT) as uint)];

  let scene = Scene {
    eye: Point::new(-12000., 0., 0.),
    spots: ~[
      Spot {
        pos: Point::new(-1000., -200., 0.),
        color: White
      },
    ],
    objects: ~[
      Object {
        pos: Point::new(0., 400., 400.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., 400., 0.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., 400., -400.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., -400., 400.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., -400., 0.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., -400., -400.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., 0., 400.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., 0., 0.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
      Object {
        pos: Point::new(0., 0., -400.),
        shininess: 0.,
        color: Green,
        shape: ~Sphere { radius: 100. }
      },
    ]
  };

  for x in range(0, WIDTH) {
    for y in range(0, HEIGHT) {
      let vector = Point {
        x: 10000.,
        y: (WIDTH / 2 - x) as f64,
        z: (HEIGHT / 2 - y) as f64
      };

      let (obj, k) = scene.get_closest(&vector);

      pixels[y * WIDTH + x] =
        if obj.is_none() || k <= 0. {
           Black
        }
        else {
          let closest = obj.unwrap();
          scene.get_color(closest, &vector, k)
        };
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