use std::{
    io::{self},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::Rng;

const TWO_PI: f64 = 2. * std::f64::consts::PI;
const EPSILON: f64 = 1e-8;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3(f64, f64, f64);

pub type Color3 = Vector3;
pub type Point3 = Vector3;

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl Mul<f64> for Vector3 {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Self(self.0 * t, self.1 * t, self.2 * t)
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, t: f64) {
        self.0 *= t;
        self.1 *= t;
        self.2 += t;
    }
}

impl Div<f64> for Vector3 {
    type Output = Self;

    fn div(self, t: f64) -> Self {
        self * (1. / t)
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, t: f64) {
        self.0 *= 1. / t;
        self.1 *= 1. / t;
        self.2 *= 1. / t;
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn rand_unit(rng: &mut rand::rngs::ThreadRng) -> Self {
        let alpha = rng.gen_range(0. ..TWO_PI);
        let beta = rng.gen_range(0. ..TWO_PI);
        Self::new(
            alpha.sin() * beta.cos(),
            alpha.sin() * beta.sin(),
            alpha.cos(),
        )
    }

    pub fn near_zero(&self) -> bool {
        self.0.abs() < EPSILON && self.1.abs() < EPSILON && self.2.abs() < EPSILON
    }

    pub fn reflect(self, normal: Self) -> Self {
        self - normal * 2. * self.dot(normal)
    }
}

fn clamp(x: f64, min: f64, max: f64) -> f64 {
    match x {
        _ if x < min => min,
        _ if x > max => max,
        _ => x,
    }
}

pub fn write_color(stream: &mut dyn io::Write, color: &Color3) -> io::Result<()> {
    let r = color.0.sqrt();
    let g = color.1.sqrt();
    let b = color.2.sqrt();
    stream.write_fmt(format_args!(
        "{} {} {}\n",
        (clamp(r, 0., 0.999) * 256.) as u32,
        (clamp(g, 0., 0.999) * 256.) as u32,
        (clamp(b, 0., 0.999) * 256.) as u32
    ))
}
