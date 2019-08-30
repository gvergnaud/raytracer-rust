use std::sync::Arc;
use vec3::{Vec3};
use ray::{Ray};
use aabb::{Aabb};
use material::{Material};

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a dyn Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) ->
        Option<Aabb>;
}

pub type HitableList = Vec<Box<dyn Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter().fold(None, |acc, item| {
            match (acc, item.hit(r, t_min, t_max)) {
                (None, None) => None,
                (Some(rec), None) => Some(rec),
                (None, Some(rec)) => Some(rec),
                (Some(acc_rec), Some(rec)) => Some(
                    if rec.t < acc_rec.t {
                        rec
                    } else {
                        acc_rec
                    }
                )
            }
        })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        self.iter().fold(None, |acc, item| {
            match (acc, item.bounding_box(t0, t1)) {
                (None, None) => None,
                (Some(box1), Some(box2)) => Some(box1.surrounding_box(&box2)),
                (None, Some(bounding_box)) => Some(bounding_box),
                (Some(bounding_box), None) => Some(bounding_box),
            }
        })
    }
}

fn hit_sphere<'a>(
    center: Vec3,
    radius: f64,
    material: &'a dyn Material,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<HitRecord<'a>> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = oc.dot(ray.direction);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - a * c;

    if discriminant > 0. {
        let t1 = (-b - (b.powi(2) - a * c).sqrt()) / a;

        if t1 < t_max && t1 > t_min {
            let point = ray.point_at_parameter(t1);
            return Some(HitRecord {
                t: t1,
                point,
                normal: (point - center) / radius,
                material,
            });
        }

        let t2 = (-b + (b.powi(2) - a * c).sqrt()) / a;

        if t2 < t_max && t2 > t_min {
            let point = ray.point_at_parameter(t2);
            return Some(HitRecord {
                t: t2,
                point,
                normal: (point - center) / radius,
                material,
            });
        }
    }
    None
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        hit_sphere(
            self.center,
            self.radius,
            &*self.material,
            ray,
            t_min,
            t_max
        )
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
        Some(
            Aabb {
                min: self.center - Vec3::fromf(self.radius),
                max: self.center + Vec3::fromf(self.radius),
            }
        )
    }
}

#[derive(Clone)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, material: Arc<dyn Material>) -> Self {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hitable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        hit_sphere(
            self.center(ray.time),
            self.radius,
            &*self.material,
            ray,
            t_min,
            t_max
        )
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
        let box0 = Aabb {
            min: self.center(t0) - Vec3::fromf(self.radius),
            max: self.center(t0) + Vec3::fromf(self.radius),
        };
        
        let box1 = Aabb {
            min: self.center(t1) - Vec3::fromf(self.radius),
            max: self.center(t1) + Vec3::fromf(self.radius),
        };

        Some(box0.surrounding_box(&box1))
    }
}
