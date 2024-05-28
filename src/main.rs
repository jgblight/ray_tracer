mod hittable;
mod material;
mod ray;
mod render;
mod vector;
use material::{DialectricMaterial, LambertianMaterial, Material, MirrorMaterial};
use rand::{rngs::ThreadRng, Rng};
use render::{Camera, Canvas};
use vector::Color3;

use crate::{
    hittable::{Sphere, World},
    vector::{write_color, Point3},
};
use std::{io, iter::Iterator};

const ASPECT_RATIO: f64 = 16. / 9.;
const IMAGE_HEIGHT: u32 = 800;

const VERTICAL_FOV: f64 = 60.;
const PIXEL_SAMPLES: usize = 100;

const FOCUS_DISTANCE: f64 = 10.; // Controls distance of virtual lens from focus plane
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

fn random_material(rng: &mut ThreadRng) -> Box<dyn Material> {
    let x = rng.gen_range(0. ..1.);
    if x < 0.6 {
        let albedo = Color3::random(rng) * Color3::random(rng);
        Box::new(LambertianMaterial { albedo })
    } else if x < 0.8 {
        let albedo = Color3::random_range(rng, 0.5, 1.);
        Box::new(MirrorMaterial {
            albedo,
            fuzziness: rng.gen_range(0. ..0.4),
        })
    } else {
        Box::new(DialectricMaterial {
            refractive_index: 1.5,
        })
    }
}

fn main() -> io::Result<()> {
    let camera = Camera::new(
        ASPECT_RATIO,
        IMAGE_HEIGHT,
        VERTICAL_FOV,
        Point3::new(13., 3., 0.),
        Point3::new(0., 1., 0.),
        DEFOCUS_ANGLE,
        FOCUS_DISTANCE,
        PIXEL_SAMPLES,
    );

    let mut rng = rand::thread_rng();
    let mut stream = io::stdout();
    let mut world = World::new();

    let ground_material = Box::new(LambertianMaterial {
        albedo: Color3::new(0.8, 0.8, 0.),
    });
    let ground = Sphere {
        center: Point3::new(0., -1000., -1.),
        radius: 1000.,
        material: (ground_material as Box<dyn Material>),
    };
    world.add(Box::new(ground));

    let mirror = Box::new(MirrorMaterial {
        albedo: Color3::new(0.8, 0.8, 0.8),
        fuzziness: 0.1,
    });
    let mirror_center = Point3::new(4., 2., 1.);
    let mirror_radius = 2.;
    let mirror_sphere = Sphere {
        center: mirror_center,
        radius: mirror_radius,
        material: (mirror as Box<dyn Material>),
    };
    world.add(Box::new(mirror_sphere));
    for i in (-2..10).step_by(3) {
        for j in (-6..7).step_by(3) {
            let radius = rng.gen_range(0.1..0.7);
            let center = Point3::new(
                i as f64 + rng.gen_range(-0.5..0.5),
                radius,
                j as f64 + rng.gen_range(-0.5..0.5),
            );

            if (center - mirror_center).length() < mirror_radius + radius {
                continue;
            }

            let material = random_material(&mut rng);
            let sphere = Sphere {
                center,
                radius,
                material: material,
            };
            world.add(Box::new(sphere));
        }
    }

    for i in (0..360).step_by(60) {
        let distance = rng.gen_range(1.5..4.);
        let height = rng.gen_range(0.8..3.);
        let theta = (i as f64).to_radians();
        let center =
            Point3::new(distance * theta.cos(), height, distance * theta.sin()) + mirror_center;
        let material = random_material(&mut rng);
        let sphere = Sphere {
            center,
            radius: rng.gen_range(0.1..0.5),
            material: material,
        };
        world.add(Box::new(sphere));
    }

    let canvas = camera.draw(&world, &mut rng);
    write_image(&mut stream, &canvas)?;
    Ok(())
}
