use std::ops::Range;
use std::option::Option;
use std::vec::Vec;

use crate::material::Material;
use crate::ray::Ray;
use crate::vector::{Point3, Vector3};

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: Point3,
    pub normal: Vector3,
    pub distance: f64,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<Hit>;
}

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<Hit> {
        // Analytically solve for the intersection between the ray and the surface of these sphere
        let sphere_to_origin = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let b = ray.direction.dot(sphere_to_origin);
        let c = sphere_to_origin.length_squared() - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant <= 0. {
            // The ray does not intersect the sphere
            return None;
        };
        let dis_sqrt = discriminant.sqrt();

        // Choose the intersection point that is closest to the ray origin while still falling in the renderable range
        let mut t = (-b - dis_sqrt) / a;
        if !range.contains(&t) {
            t = (-b + dis_sqrt) / a;
            if !range.contains(&t) {
                return None;
            }
        }
        let point = ray.at(t);
        let normal = (point - self.center) / self.radius;

        let intersection = Hit {
            point: point,
            normal: normal,
            distance: t,
            material: self.material,
        };
        Some(intersection)
    }
}

pub struct World<'a> {
    shapes: Vec<&'a dyn Hittable>,
}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    pub fn add(&mut self, elem: &'a dyn Hittable) {
        self.shapes.push(elem);
    }
}

impl Hittable for World<'_> {
    fn hit(&self, ray: &Ray, range: &Range<f64>) -> Option<Hit> {
        // For all objects in the world, return the valid hit that is closes to the camera
        let mut closest_hit: Option<Hit> = None;

        for shape in &self.shapes {
            if let Some(hit) = shape.hit(ray, range) {
                if closest_hit.is_none() || closest_hit.is_some_and(|x| x.distance > hit.distance) {
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}
