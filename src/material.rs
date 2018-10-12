extern crate rand;
use ray::{Ray};
use hitable::{HitRecord};
use vec3::{Vec3};

pub struct MaterialRecord {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<MaterialRecord>;
}

fn random_point_in_unit_sphere() -> Vec3 {
    let mut point: Vec3;
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y: f64 = rng.gen();
    let z: f64 = rng.gen();
    loop {
        point = Vec3::new(x, y, z);
        if point.length() < 1. { break; }
    }
    point
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<MaterialRecord> {
        Some(MaterialRecord {
            scattered: Ray {
                origin: rec.point,
                direction: rec.point + rec.normal + random_point_in_unit_sphere(),
            },
            attenuation: self.albedo,
        })

    }
}
