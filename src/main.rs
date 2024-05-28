mod hittable;
mod material;
mod ray;
mod render;
mod vector;
use material::{DialectricMaterial, LambertianMaterial, MirrorMaterial};
use render::{Camera, Canvas};
use vector::Color3;

use crate::{
    hittable::{Sphere, World},
    vector::{write_color, Point3},
};
use std::io::{self};

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_HEIGHT: u32 = 400;

const VERTICAL_FOV: f64 = 50.;
const PIXEL_SAMPLES: usize = 100;

const FOCUS_DISTANCE: f64 = 3.; // Controls distance of virtual lens from focus plane
const DEFOCUS_ANGLE: f64 = 0.6; // Controls size of virtual lens

fn write_image(stream: &mut dyn io::Write, canvas: &Canvas) -> io::Result<()> {
    stream.write_all(format!("P3\n{} {}\n255\n", canvas.width, canvas.height).as_bytes())?;
    for j in 0..canvas.height {
        for i in 0..canvas.width {
            write_color(stream, &canvas.get_pixel(i, j))?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let mut stream = io::stdout();

    let mut world = World::new();

    let ground_material = LambertianMaterial {
        albedo: Color3::new(0.8, 0.8, 0.),
    };
    let red_lambert = LambertianMaterial {
        albedo: Color3::new(0.7, 0.2, 0.2),
    };
    let mirror = MirrorMaterial {
        albedo: Color3::new(0.8, 0.8, 0.8),
        fuzziness: 0.1,
    };
    let ground = Sphere {
        center: Point3::new(0., -100.5, -1.),
        radius: 100.,
        material: &ground_material,
    };
    world.add(&ground);
    let sphere = Sphere {
        center: Point3::new(0., 0., -1.2),
        radius: 0.5,
        material: &mirror,
    };
    world.add(&sphere);
    let sphere2 = Sphere {
        center: Point3::new(-0.5, -0.2, -0.6),
        radius: 0.1,
        material: &red_lambert,
    };
    world.add(&sphere2);

    let glass = DialectricMaterial {
        refractive_index: 1.5,
    };
    let sphere3 = Sphere {
        center: Point3::new(0.3, -0.3, -0.7),
        radius: 0.15,
        material: &glass,
    };
    world.add(&sphere3);

    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_HEIGHT,
        VERTICAL_FOV,
        Point3::new(-1., 2., 1.),
        Point3::new(0., 0., -1.),
        DEFOCUS_ANGLE,
        FOCUS_DISTANCE,
        PIXEL_SAMPLES,
    );
    let mut rng = rand::thread_rng();

    let canvas = camera.draw(&world, &mut rng);
    write_image(&mut stream, &canvas)?;
    Ok(())
}
