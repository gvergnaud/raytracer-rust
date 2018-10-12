use hitable::{Hitable, HitRecord};
use ray::{Ray};

pub type HitableList = Vec<Box<Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        self.iter().fold(None, |acc, item| {
            match item.hit(r, t_min, closest_so_far) {
                None => acc,
                Some(rec) => {
                    if rec.t < closest_so_far {
                        closest_so_far = rec.t;
                        Some(rec)
                    } else {
                        acc
                    }
                }
            }
        })
    }
}
