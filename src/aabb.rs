use vec3::Vec3;
use ray::Ray;

// for axis aligned bounding box
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
  pub min: Vec3,
  pub max: Vec3,
}

impl Aabb {
  pub fn hit(&self, r: &Ray, mut tmin: f64, mut tmax: f64) -> bool {
    for a in 0..3 {
      let min_t = (self.min[a] - r.origin[a]) / r.direction[a];
      let max_t = (self.max[a] - r.origin[a]) / r.direction[a];
      let t0 = ffmin(min_t, max_t);
      let t1 = ffmax(min_t, max_t);

      tmin = ffmax(t0, tmin);
      tmax = ffmin(t1, tmax);

      if tmax <= tmin {
        return false;
      }
    }

    true
  }

  pub fn surrounding_box(&self, aabb: &Aabb) -> Aabb {
    surrounding_box(self, aabb)
  }
}

pub fn surrounding_box(aabb1: &Aabb, aabb2: &Aabb) -> Aabb {
  Aabb {
    min: Vec3 {
      x: aabb1.min.x.min(aabb2.min.x),
      y: aabb1.min.y.min(aabb2.min.y),
      z: aabb1.min.z.min(aabb2.min.z),
    },
    max: Vec3 {
      x: aabb1.max.x.max(aabb2.max.x),
      y: aabb1.max.y.max(aabb2.max.y),
      z: aabb1.max.z.max(aabb2.max.z),
    }
  }
}

pub fn optional_surrounding_box(maybe_aabb0: Option<Aabb>,maybe_aabb1: Option<Aabb>) -> Option<Aabb> {
  match (maybe_aabb0, maybe_aabb1) {
    (Some(aabb0), Some(aabb1)) => Some(aabb0.surrounding_box(&aabb1)),
    (Some(aabb), None) => Some(aabb),
    (None, Some(aabb)) => Some(aabb),
    (None, None) => None,
  }
}

fn ffmax(a: f64, b: f64) -> f64 {
  if a > b { a } else { b }
}

fn ffmin(a: f64, b: f64) -> f64 {
  if a < b { a } else { b }
}