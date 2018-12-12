use vec3::Vec3;
use ray::Ray;

// for axis aligned bounding box
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
    Aabb {
      min: Vec3 {
        x: self.min.x.min(aabb.min.x),
        y: self.min.y.min(aabb.min.y),
        z: self.min.z.min(aabb.min.z),
      },
      max: Vec3 {
        x: self.max.x.max(aabb.max.x),
        y: self.max.y.max(aabb.max.y),
        z: self.max.z.max(aabb.max.z),
      }
    }
  }
}

fn ffmax(a: f64, b: f64) -> f64 {
  if a > b { a } else { b }
}

fn ffmin(a: f64, b: f64) -> f64 {
  if a < b { a } else { b }
}