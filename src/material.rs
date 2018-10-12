extern crate rand;
use self::rand::Rng;
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

    loop {
        point = Vec3::new(rng.gen(), rng.gen(), rng.gen());
        if point.length() < 1. { break; }
    }
    point
}

fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - 2. * vec.dot(normal) * normal
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
    fn scatter(&self, _r: &Ray, rec: &HitRecord) -> Option<MaterialRecord> {
        Some(MaterialRecord {
            scattered: Ray {
                origin: rec.point,
                direction: rec.normal + random_point_in_unit_sphere(),
            },
            attenuation: self.albedo,
        })

    }
}


pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<MaterialRecord> {
        let reflected = reflect(r.direction.unit_vector(), rec.normal);

        let scattered = Ray {
            origin: rec.point,
            direction: reflected + self.fuzz * random_point_in_unit_sphere()
        };

        if scattered.direction.dot(rec.normal) > 0. {
            Some(MaterialRecord {
                scattered,
                attenuation: self.albedo
            })
        } else {
            None
        }
    }
}
