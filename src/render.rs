use rand::{rngs::ThreadRng, Rng};
use std::{collections::HashMap, ops::Range};

use crate::{
    hittable::Hittable,
    ray::Ray,
    vector::{Color3, Point3, Vector3},
};

const MAX_BOUNCE_DEPTH: usize = 10;

// Resolve the color returned by a single ray by simulating it bouncing and scattered off objects in the scene
fn compute_ray(
    ray: &Ray,
    world: &dyn Hittable,
    rng: &mut rand::rngs::ThreadRng,
    max_depth: usize,
) -> Color3 {
    if max_depth == 0 {
        return Color3::new(0., 0., 0.);
    }

    let hit = world.hit(
        ray,
        &Range {
            start: 0.01,
            end: f64::INFINITY,
        },
    );
    match hit {
        Some(h) => {
            // If the ray hits something, it will bounce off in a random direction
            let scattered = h.material.scatter(ray, &h, rng);
            match scattered {
                Some(s) => compute_ray(&s.ray, world, rng, max_depth - 1) * s.attentuation,
                None => Color3::new(0., 0., 0.)
            }   
        }
        None => {
            // If the ray hits nothing, return a sky colour
            let a = ray.direction.y() * 0.5 + 1.;
            Color3::new(1., 1., 1.) * (1. - a) + Color3::new(0.5, 0.7, 1.) * a
        }
    }
}

// Interface for
// We define the coordinate space so that x is right, y is up and the viewport is in the negative z direction from the camera
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    aspect_ratio: f64,
    image_height: u32,
    image_width: u32,

    focal_length: f64, // distance from camera to viewport
    camera_center: Point3,

    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,

    pixel_00: Vector3,
    samples: usize,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_height: u32,
        viewport_height: f64,
        focal_length: f64,
        samples: usize,
    ) -> Self {
        let image_width = (image_height as f64 * aspect_ratio) as u32;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Viewport setup
        let camera_center = Point3::new(0., 0., 0.);
        let viewport_u = Vector3::new(viewport_width, 0., 0.); // vector along width of viewport
        let viewport_v = Vector3::new(0., -viewport_height, 0.); // vector along height of viewport

        // The viewport is subdivided into pixels, where the color of the pixel is determined by the ray drawn through its center
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Find upper left corner of viewport of (-V_u / 2, V_v / 2)
        let viewport_origin = camera_center
            - Vector3::new(0., 0., focal_length)
            - (viewport_u / 2.)
            - (viewport_v / 2.);
        let pixel_00 = viewport_origin + (pixel_delta_u / 2.) + (pixel_delta_v / 2.);
        Self {
            aspect_ratio,
            image_height,
            image_width,
            focal_length,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel_00,
            samples,
        }
    }

    pub fn draw(self, world: &dyn Hittable, rng: &mut ThreadRng) -> Canvas {
        let mut canvas = Canvas::new(self.image_width, self.image_height);
        for i in 0..canvas.width {
            for j in 0..canvas.height {
                let color = self.draw_pixel(i, j, world, rng);
                canvas.put_pixel(i, j, color);
            }
        }
        canvas
    }

    fn draw_pixel(self, i: u32, j: u32, world: &dyn Hittable, rng: &mut ThreadRng) -> Color3 {
        // Sample a collection of rays within the pixel and take the average color
        let pixel_center =
            self.pixel_00 + (self.pixel_delta_u * i as f64) + (self.pixel_delta_v * j as f64);
        let mut color = Color3::new(0., 0., 0.);
        for _ in 0..self.samples {
            let pixel_offset = (self.pixel_delta_u * rng.gen_range(-0.5..0.5))
                + (self.pixel_delta_v * rng.gen_range(-0.5..0.5));
            let ray_direction = pixel_center + pixel_offset - self.camera_center;
            let ray = Ray::new(self.camera_center, ray_direction);
            color += compute_ray(&ray, world, rng, MAX_BOUNCE_DEPTH);
        }
        color /= self.samples as f64;
        color
    }
}

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pixels: HashMap<(u32, u32), Color3>,
    default: Color3,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: HashMap::new(),
            default: Color3::new(0., 0., 0.),
        }
    }

    pub fn get_pixel<'a>(&'a self, x: u32, y: u32) -> &'a Color3 {
        match self.pixels.get(&(x, y)) {
            Some(c) => c,
            None => &self.default,
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, color: Color3) {
        self.pixels.insert((x, y), color);
    }
}
