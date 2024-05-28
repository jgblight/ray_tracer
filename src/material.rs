use rand::rngs::ThreadRng;
use rand::Rng;

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
    pub fuzziness: f64,
}

impl Material for MirrorMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<ScatteredHit> {
        let reflected = ray.direction.reflect(hit.normal);
        let bounce_direction = reflected + Vector3::rand_unit(rng) * self.fuzziness;
        if bounce_direction.dot(hit.normal) > 0. {
            let bounce_ray = Ray::new(hit.point, bounce_direction);
            Some(ScatteredHit::new(bounce_ray, self.albedo))
        } else {
            None
        }
    }
}

pub struct DialectricMaterial {
    pub refractive_index: f64,
}

fn reflectance(cos_theta: f64, refraction_index: f64) -> f64 {
    let r_root = (1. - refraction_index) / (1. + refraction_index);
    let r = r_root * r_root;
    r + (1. - r) * (1. - cos_theta).powi(5)
}

impl Material for DialectricMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> Option<ScatteredHit> {
        let (refraction_ratio, normal) = if ray.direction.dot(hit.normal) < 0. {
            // hitting front face
            (1. / self.refractive_index, hit.normal)
        } else {
            // leaving back face
            (self.refractive_index, -hit.normal)
        };

        let cos_theta = normal.dot(-ray.direction);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let bounce_direction = if sin_theta * refraction_ratio > 1.
            || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.)
        {
            // reflect
            ray.direction.reflect(normal)
        } else {
            // refract
            let refracted_perpendicular = (ray.direction + normal * cos_theta) * refraction_ratio;
            let refracted_parallel =
                normal * -((1. - refracted_perpendicular.length_squared()).abs().sqrt());
            refracted_parallel + refracted_perpendicular
        };

        Some(ScatteredHit::new(
            Ray::new(hit.point, bounce_direction),
            Color3::new(1., 1., 1.),
        ))
    }
}
