use hitable::{Hitable, HitRecord};
use ray::{Ray};

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
