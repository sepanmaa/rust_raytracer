use std::io::prelude::*;
use std::fs::File;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

#[macro_use]
mod vector;

mod raytracer;

use vector::Vector3;

use raytracer::*;


fn main() {
    let mut scene = scene();
    
    scene.camera = Camera {
        pos: v3!(0.5, 2.5, -1.0),
        up: v3!(0.0, 1.0, 0.2).normalize(),
        right: v3!(1.33, 0.0, 0.0),
        dist: 2.0,
    };

    let mut red = basic_material(v3!(1.0, 0.0, 0.0));
    red.shininess = 64.0;
    let blue = basic_material(v3!(0.0, 0.0, 1.0));
    let green = basic_material(v3!(0.0, 1.0, 0.0));
    let mirror = Material { shininess: 32.0,
                            spec_color: v3!(1.0, 1.0, 1.0),
                            color: v3!(1.0, 1.0, 1.0),
                            reflection: 0.7 };

    scene.add(Sphere { pos: v3!(-2.0, 1.5, 7.0), radius: 0.5, material: red });
    scene.add(Sphere { pos: v3!(-1.0, -0.5, 8.0), radius: 0.5, material: blue });
    scene.add(Sphere { pos: v3!(-3.0, -0.5, 5.0), radius: 0.5, material: green });
    scene.add(Plane { pos: v3!(0.0, -1.0, 0.0), normal: v3!(0.0, 1.0, 0.0), material: red });
    scene.add(BBox { v1: v3!(-2.5, -1.0, 6.0), v2: v3!(-1.5, 1.0, 10.0), material: mirror });
    scene.add(BBox { v1: v3!(2.0, -1.0, 5.0), v2: v3!(3.0, 1.0, 6.0), material: green });
    scene.add(Sphere { pos: v3!(1.0, 0.0, 8.0), radius: 1.0, material: mirror });
    scene.lights = vec![Light { pos: v3!(20.0, 20.0, -20.0), color: v3!(1.0, 1.0, 1.0)}];
                                      
    let pixels = raytrace(&scene, WIDTH, HEIGHT);

    let mut f = File::create("raytracing.ppm").expect("Could not create file."); 
    let mut ppm: Vec<String> = Vec::new();
    ppm.push(format!("P3 {} {} 255", WIDTH, HEIGHT));
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let (r, g, b) = pixels[y*WIDTH+x].to_rgb();
            ppm.push(format!("{} {} {}", r, g, b));
        }
    }
    f.write_fmt(format_args!("{}\n", ppm.join(" "))).ok();    
}
