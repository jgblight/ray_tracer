use rand::rngs::ThreadRng;

use crate::{
    hittable::Hit,
    ray::Ray,
    vector::{Color3, Vector3},
};

pub struct ScatteredHit {
    pub ray: Ray,
    pub attentuation: Color3,
}

impl ScatteredHit {
    pub fn new(ray: Ray, attentuation: Color3) -> Self {
        Self { ray, attentuation }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<ScatteredHit>;
}

// Lambert or "matte" material bounces light in a random direction
pub struct LambertianMaterial {
    pub albedo: Color3,
}

impl Material for LambertianMaterial {
    fn scatter(&self, _ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<ScatteredHit> {
        let bounce_direction = hit.normal + Vector3::rand_unit(rng);
        let bounce_ray = Ray::new(
            hit.point,
            if bounce_direction.near_zero() {
                hit.normal
            } else {
                bounce_direction
            },
        );
        Some(ScatteredHit::new(bounce_ray, self.albedo))
    }
}


pub struct MirrorMaterial {
    pub albedo: Color3,
    pub fuzziness: f64
}

impl Material for MirrorMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<ScatteredHit> {
        let reflected = ray.direction.reflect(hit.normal);
        let bounce_direction = reflected + Vector3::rand_unit(rng)*self.fuzziness;
        if bounce_direction.dot(hit.normal) > 0. {
            let bounce_ray = Ray::new(hit.point, bounce_direction);
            Some(ScatteredHit::new(bounce_ray, self.albedo))
        } else {
            None
        }
    }
}