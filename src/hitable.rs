use vec3::{Vec3};
use ray::{Ray};

pub struct HitRecord {
    t: f64,
    point: Vec3,
    normal: Vec3,
}

pub trait Hitable {
    fn hit(self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
