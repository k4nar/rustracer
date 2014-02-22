#[ crate_id = "rustracer#0.1" ];

#[ desc = "Rustracer" ];
#[ license = "Apache v2" ];
#[ comment = "A pretty dumb raytracer, in Rust." ];

#[crate_type = "bin"];

extern crate png;
mod color;
mod point;
mod scene;
mod shapes;
mod main;
