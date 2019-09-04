use vec3::{Vec3};
use noises::{Perlin};

pub trait Texture {
  fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3;
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
  fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
    let sines = (10. * p.x).sin() * (10. * p.y).sin() + (10. * p.z).sin();
    if sines < 0. {
      self.odd.value(u, v, p)
    } else {
      self.even.value(u, v, p)
    }
  }
}

pub struct NoiseTexture {
  noise: Perlin,
  scale: f32
}

impl NoiseTexture {
  pub fn new(scale: f32) -> Self {
    NoiseTexture {
      noise: Perlin::new(),
      scale
    }
  }
}

impl Texture for NoiseTexture {
  fn value(&self, _u: f32, _v: f32, p: Vec3) -> Vec3 {
    // light dense marble
    // return Vec3::fromf(1.) * 0.5 * (1. + self.noise.turb(self.scale * p));

    // dark dense marble
    // return Vec3::fromf(1.) * self.noise.turb(self.scale * p);

    // realistic marble
    Vec3::fromf(1.) *
    0.5 *
    (1. + (self.scale * p.z + 10. * self.noise.turb(p)).sin())
  }
}
