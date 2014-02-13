use png;
use color::{Color, Black, White, Red, Green, Blue};
use math3d::Point;
use scene::{Scene, Spot};
use shapes::{Shape, Sphere, Plane};

static WIDTH: int = 800;
static HEIGHT: int = 800;

#[main]
fn main() {
  let mut pixels = [Black, ..((WIDTH * HEIGHT) as uint)];

  let scene = Scene {
    eye: Point::new(-300., 0., 200.),
    spot: Spot {
      pos: Point::new(-300., 100., 200.),
      color: White
    },
    objects: ~[
      Shape {
        pos: Point::new(0., 0., 100.),
        shininess: 0.2,
        color: Red,
        shape: ~Sphere { radius: 160. }
      },
      Shape {
        pos: Point::new(0., 0., 0.),
        shininess: 0.1,
        color: Green,
        shape: ~Plane
      },
    ]
  };

  for x in range(0, WIDTH) {
    for y in range(0, HEIGHT) {
      let vector = Point {
        x: 100.,
        y: (WIDTH / 2 - x) as f64,
        z: (HEIGHT / 2 - y) as f64
      };

      let (obj, k) = scene.get_closest(&vector);

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

        let inter = Point {
          x: (scene.eye.x - closest.pos.x) + k * vector.x,
          y: (scene.eye.y - closest.pos.y) + k * vector.y,
          z: (scene.eye.z - closest.pos.z) + k * vector.z
        };
        let light = Point {
          x: scene.spot.pos.x - inter.x,
          y: scene.spot.pos.y - inter.y,
          z: scene.spot.pos.z - inter.z
        };

        pixels[y * WIDTH + x] = match k {
          0. => Black,
          _ => closest.get_light(&scene, &inter, &light)
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