use hitable::{Hitable, HitRecord};
use ray::{Ray};

pub type HitableList = Vec<Box<Hitable>>;

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.iter().fold(None, |acc, item| {
            match acc {
                Some(x) => Some(x),
                None => {
                    match item.hit(r, t_min, t_max) {
                        Some(x) => Some(x),
                        None => None,
                    }
                }
            }
        })
    }
}
