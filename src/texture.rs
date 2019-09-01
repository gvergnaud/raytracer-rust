use vec3::{Vec3};

pub trait Texture {
  fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3;
}

pub struct ConstantTexture {
  color: Vec3
}

impl ConstantTexture {
  pub fn new(color: Vec3) -> Self {
    ConstantTexture {
      color
    }
  }
}

impl Texture for ConstantTexture {
  fn value(&self, _u: f32, _v: f32, _p: Vec3) -> Vec3 {
    self.color
  }
}


pub struct CheckedTexture {
  pub even: Box<ConstantTexture>,
  pub odd : Box<ConstantTexture>
}

impl CheckedTexture {
  pub fn new(even: Box<ConstantTexture>, odd: Box<ConstantTexture>) -> Self {
    CheckedTexture {
      even,
      odd
    }
  }
}

impl Texture for CheckedTexture {
  fn value(&self, u: f32, v: f32, point: Vec3) -> Vec3 {
    let sines = (10. * point.x).sin() * (10. * point.y).sin() + (10. * point.z).sin();
    if sines < 0. {
      self.odd.value(u, v, point)
    } else {
      self.even.value(u, v, point)
    }
  }
}