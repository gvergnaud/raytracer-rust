use std::sync::Arc;
use vec3::{Vec3};
use ray::{Ray};
use material::{Material};

pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type HitableList = Vec<Box<Hitable>>;

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
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0. {
            let t1 = (-b - (b.powi(2) - a * c).sqrt()) / a;

            if t1 < t_max && t1 > t_min {
                let point = r.point_at_parameter(t1);
                return Some(HitRecord {
                    t: t1,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: &*self.material,
                });
            }

            let t2 = (-b + (b.powi(2) - a * c).sqrt()) / a;

            if t2 < t_max && t2 > t_min {
                let point = r.point_at_parameter(t2);
                return Some(HitRecord {
                    t: t2,
                    point,
                    normal: (point - self.center) / self.radius,
                    material: &*self.material,
                });
            }
        }
        None
    }
}
