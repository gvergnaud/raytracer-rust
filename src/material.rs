extern crate rand;
use self::rand::Rng;
use ray::{Ray};
use hitable::{HitRecord};
use vec3::{Vec3};
use texture::{ConstantTexture};

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

fn refract(vec: Vec3, normal: Vec3, refraction_indices_ratio: f64) -> Option<Vec3> {
    let uv = vec.unit_vector();
    let dt = uv.dot(normal);
    let discriminant =
        1.0 - refraction_indices_ratio.powi(2) * (1.0 - dt.powi(2));

    if discriminant > 0. {
        Some(
            refraction_indices_ratio * (uv - normal * dt) - normal * discriminant.sqrt()
        )
    } else {
        None
    }
}

#[derive(Copy, Clone)]
pub struct MaterialRecord {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<MaterialRecord>;
}

#[derive(Copy, Clone)]
pub struct Lambertian<'a> {
    pub albedo: &'a ConstantTexture
}

impl<'a> Lambertian<'a> {
    pub fn new(albedo: &'a ConstantTexture) -> Self {
        Lambertian {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<MaterialRecord> {
        Some(MaterialRecord {
            scattered: Ray {
                origin: rec.point,
                direction: rec.normal + random_point_in_unit_sphere(),
                time: ray.time,
            },
            attenuation: self.albedo,
        })

    }
}

#[derive(Copy, Clone)]
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
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<MaterialRecord> {
        let reflected = reflect(ray.direction.unit_vector(), rec.normal);

        let scattered = Ray {
            origin: rec.point,
            direction: reflected + self.fuzz * random_point_in_unit_sphere(),
            time: ray.time,
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

#[derive(Copy, Clone)]
pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric {
            refraction_index,
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<MaterialRecord> {
        let is_ray_inside_object = ray.direction.dot(rec.normal) > 0.;

        // outward normal is the normal pointing in the opposite
        // direction of the ray
        let outward_normal =
            if is_ray_inside_object { -rec.normal }
            else { rec.normal };

        let refraction_ratio =
            if is_ray_inside_object { self.refraction_index }
            else { 1. / self.refraction_index };


        match refract(
            ray.direction,
            outward_normal,
            refraction_ratio
        ) {
            Some(refracted) => Some(
                MaterialRecord {
                    scattered: Ray {
                        origin: rec.point,
                        direction: refracted,
                        time: ray.time,
                    },
                    attenuation: Vec3::fromf(1.),
                }
            ),
            None => Some(
                MaterialRecord {
                    scattered: Ray {
                        origin: rec.point,
                        direction: reflect(ray.direction, rec.normal),
                        time: ray.time,
                    },
                    attenuation: Vec3::fromf(1.),
                }
            ),
        }
    }
}
