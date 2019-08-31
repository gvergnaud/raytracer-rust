// bvh = bounding volume hierarchy. It's a tree containing bounding volumes, used to quickly detect a hit by using binary search, as far as I understand it right now.

use std::cmp::Ordering;
use rand::{Rng, self};
use ray::{Ray};
use aabb::{Aabb, self};
use hitable::{Hitable, HitRecord};

#[derive(Debug, Copy, Clone)]
pub struct NodeId {
  index: usize
}

pub struct BvhTree<'a> {
  nodes: Vec<BvhNode<'a>>,
  root: NodeId
}

pub struct BvhNode<'a> {
  left: Option<NodeId>,
  right: Option<NodeId>,
  aabb: Option<Aabb>,
  hitable: Option<&'a Box<dyn Hitable>>
}

impl<'a> Hitable for BvhNode<'a> {
  fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<Aabb> {
    self.aabb
  }

  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    match self.hitable {
      Some(hitable) => hitable.hit(r, t_min, t_max),
      None => None
    }
  }
}

impl<'a> BvhTree<'a> {
  pub fn new(list: &'a mut[Box<dyn Hitable>], time0: f64, time1: f64) -> BvhTree {
    let mut tree = BvhTree {
      nodes: Vec::new(),
      root: NodeId { index: 0 }
    };
    tree.root = tree.build(list, time0, time1);
    tree
  }

  fn build(&mut self, list: &'a mut[Box<dyn Hitable>], time0: f64, time1: f64) -> NodeId {
    let axis = rand::thread_rng().gen_range::<i32>(0, 3);
    
    match axis {
      0 => list.sort_by(|a, b| box_x_compare(a, b)),
      1 => list.sort_by(|a, b| box_y_compare(a, b)),
      2 => list.sort_by(|a, b| box_z_compare(a, b)),
      _ => panic!("Random axis out of range"),
    };

    match list.len() {
      1 => {
        self.new_leaf(&list[0], time0, time1)
      },
      2 => {
        let left = self.new_leaf(&list[0], time0, time1);
        let right = self.new_leaf(&list[1], time0, time1);
        let box_left = self.nodes[left.index].bounding_box(time0, time1);
        let box_right = self.nodes[right.index].bounding_box(time0, time1);

        self.new_node(
          BvhNode {
            left: Some(left),
            right: Some(right),
            aabb: aabb::optional_surrounding_box(box_left, box_right),
            hitable: None
          }
        )
      },
      len => {
        let half_len = len / 2;
        let (left_list, right_list) = list.split_at_mut(half_len);
        let left = self.build(left_list, time0, time1);
        let right = self.build(right_list, time0, time1);
        let box_left = self.nodes[left.index].bounding_box(time0, time1);
        let box_right = self.nodes[right.index].bounding_box(time0, time1);

        self.new_node(
          BvhNode {
            left: Some(left),
            right: Some(right),
            aabb: aabb::optional_surrounding_box(box_left, box_right),
            hitable: None
          }
        )
      }
    }
  }

  fn new_leaf (&mut self, hitable: &'a Box<dyn Hitable>, time0: f64, time1: f64) -> NodeId {
    let node = BvhNode {
      left: None,
      right: None,
      aabb: hitable.bounding_box(time0, time1),
      hitable: Some(hitable),
    };
    self.new_node(node)
  }

  fn new_node(&mut self, node: BvhNode<'a>) -> NodeId {
    let index = self.nodes.len();
    self.nodes.push(node);
    NodeId { index }
  }

  fn hit_node(&self, node_id: NodeId, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let node = &self.nodes[node_id.index];
    if let Some(aabb) = node.aabb {
      if aabb.hit(r, t_min, t_max) {
        match (node.left, node.right) {
          (Some(left), Some(right)) => {
            match (
              self.hit_node(left, r, t_min, t_max),
              self.hit_node(right, r, t_min, t_max)
            ) {
              (Some(left_rec), Some(right_rec)) => 
                if left_rec.t < right_rec.t {
                  Some(left_rec)
                } else {
                  Some(right_rec)
                },
              (Some(left_rec), None) => Some(left_rec),
              (None, Some(right_rec)) => Some(right_rec),
              (None, None) => None,
            }
          },
          (Some(left), None) => self.hit_node(left, r, t_min, t_max),
          (None, Some(right)) => self.hit_node(right, r, t_min, t_max),
          (None, None) => node.hit(r, t_min, t_max)
        }
      } else {
        None
      }
    } else {
      None
    }
  }
}

impl<'a> Hitable for BvhTree<'a> {
  fn bounding_box(&self, t0: f64, t1: f64) -> Option<Aabb> {
    self.nodes[self.root.index].bounding_box(t0, t1)
  }
  
  fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    self.hit_node(self.root, r, t_min, t_max)
  }
}

fn box_x_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
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

fn box_y_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
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

fn box_z_compare(a: &Box<dyn Hitable>, b: &Box<dyn Hitable>) -> Ordering {
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