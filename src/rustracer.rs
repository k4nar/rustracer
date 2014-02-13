#[ crate_id = "rustracer#0.1" ];

#[ desc = "Rustracer" ];
#[ license = "Apache v2" ];
#[ comment = "A pretty dumb raytracer, in Rust." ];

#[crate_type = "bin"];

extern mod png;
mod color;
mod math3d;
mod scene;
mod shapes;
mod main;
