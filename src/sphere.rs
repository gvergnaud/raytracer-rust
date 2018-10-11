use hitable::{Hitable, HitRecord};
use vec3::{Vec3};
use ray::{Ray};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - a * c;

        if discriminant > 0. {
            let t1 = (-b - (b * b - a * c).sqrt()) / a;

            if t1 < t_max && t1 > t_min {
                let point = r.point_at_parameter(t1);
                return Some(HitRecord {
                    t: t1,
                    point,
                    normal: (point - self.center) / self.radius,
                });
            }

            let t2 = (-b + (b * b - a * c).sqrt()) / a;

            if t2 < t_max && t2 > t_min {
                let point = r.point_at_parameter(t2);
                return Some(HitRecord {
                    t: t2,
                    point,
                    normal: (point - self.center) / self.radius,
                });
            }
        }
        None
    }
}
