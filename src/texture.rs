use vec3::{Vec3};
use noises::{Perlin};
use std::f32::consts::PI;
use image;
use std::path::Path;

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


fn get_sphere_uv(p: &Vec3) -> (f32, f32) {
  // phi "left-right" angle of the point on the sphere
  let phi =  p.z.atan2(p.x);
  // theta is the up-down angle of the point on the sphere
  let theta = p.y.asin();

  let uv = (
    1. - (phi + PI) / (2. * PI),
    (theta + PI / 2.) / PI
  );

  uv
}

fn load_image(path: String) -> Vec<u8> {
  let im = image::open(Path::new(path)).unwrap().to_rbg();
  let rgb: Vec<u8> = im.raw_pixels();
  rgb
}

struct ImageTexture {
  data: Vec<u8>,
  width: f32,
  height: f32
}

impl Texture for ImageTexture {
  fn value(&self, u: f32, v: f32, p: Vec3) -> Vec3 {
    let mut i = u * self.width;
    let mut j = (1. - v) * self.height - 0.001;
    if i < 0. {
      i = 0.;
    }
    if j < 0. {
      j = 0.;
    }
    let pixel_offset = 3. * i + 3. * self.width * j;

    let r = (self.data[(pixel_offset) as usize]) as f32 / 255.0;
    let g = (self.data[(pixel_offset + 1.) as usize]) as f32 / 255.0;
    let b = (self.data[(pixel_offset + 2.) as usize]) as f32 / 255.0;

    Vec3::new(r, g, b)
  }
}
