use vec3::{Vec3};

trait Texture {
  fn value(&self, u: f64, v: f64, p: Vec3) -> Vec3;
}

pub struct ConstantTexture {
  color: Vec3
}

impl Texture for ConstantTexture {
  fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Vec3 {
    self.color
  }
}