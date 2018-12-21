// bvh = bounding volume hierarchy. It's a tree containing bounding volumes, used to quickly detect a hit by using binary search, as far as I understand it right now.

use std::cmp::Ordering;
use rand::{Rng, self};
use ray::{Ray};
use aabb::{Aabb, self};
use hitable::{Hitable, HitRecord, HitableList};

pub struct BvhNode {
  left: Box<Hitable>,
  right: Box<Hitable>,
  aabb: Aabb,
}

impl BvhNode {
  fn new(mut list: HitableList, time0: f64, time1: f64) -> BvhNode {
    let mut rng = rand::thread_rng();
    let axis = rng.gen_range::<i32>(0, 3);
    
    match axis {
      0 => list.sort_by(|a, b| box_x_compare(a, b)),
      1 => list.sort_by(|a, b| box_y_compare(a, b)),
      2 => list.sort_by(|a, b| box_z_compare(a, b)),
      _ => panic!("Random axis out of range"),
    }

    match list.len() {
      1 => {
        if let Some(aabb) = list[0].bounding_box(time0, time1) {
          BvhNode {
            left: list[0],
            right: list[0],
            aabb,
          }
        } else {
          panic!("no bounding box")
        }
      },
      2 => {
        if let Some(aabb) = aabb::optional_surrounding_box(
          list[0].bounding_box(time0, time1),
          list[1].bounding_box(time0, time1)
        ) {
          BvhNode {
            left: list[0],
            right: list[1],
            aabb,
          }
        } else {
          panic!("no bounding box")
        }
      },
      len => {
        let half_len = len / 2;
        let (left_list, right_list) = list.split_at(half_len);
        let left = BvhNode::new(left_list.to_vec(), time0, time1);
        let right = BvhNode::new(right_list.to_vec(), time0, time1);

        if let Some(aabb) = aabb::optional_surrounding_box(
          left.bounding_box(time0, time1),
          right.bounding_box(time0, time1)
        ) {
          BvhNode {
            left: Box::new(left),
            right: Box::new(right),
            aabb,
          }
        } else {
          panic!("no bounding box")
        }
      }
    }
  }
}

impl Hitable for BvhNode {
  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    Some(self.aabb)
  }

  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    if self.aabb.hit(r, t_min, t_max) {
      match (
        self.left.hit(r, t_min, t_max),
        self.right.hit(r, t_min, t_max)
      ) {
        (Some(left_rec), Some(right_rec)) => {
          if left_rec.t < right_rec.t {
            Some(left_rec)
          } else {
            Some(right_rec)
          }
        },
        (Some(left_rec), None) => Some(left_rec),
        (None, Some(right_rec)) => Some(right_rec),
        (None, None) => None,
      }
    } else {
      None
    }
  }
}

fn box_x_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
    (Some(aabb0), Some(aabb1)) => {
      if let Some(cmp) = aabb0.min.x.partial_cmp(&aabb1.min.x) {
        cmp
      } else {
        panic!("Can't compare");
      }
    },
    _ => {
      panic!("Can't compare");
    }
  }
}

fn box_y_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
    (Some(aabb0), Some(aabb1)) => {
      if let Some(cmp) = aabb0.min.y.partial_cmp(&aabb1.min.y) {
        cmp
      } else {
        panic!("Can't compare");
      }
    },
    _ => {
      panic!("Can't compare");
    }
  }
}

fn box_z_compare(a: &Box<Hitable>, b: &Box<Hitable>) -> Ordering {
  match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
    (Some(aabb0), Some(aabb1)) => {
      if let Some(cmp) = aabb0.min.z.partial_cmp(&aabb1.min.z) {
        cmp
      } else {
        panic!("Can't compare");
      }
    },
    _ => {
      panic!("Can't compare");
    }
  }
}