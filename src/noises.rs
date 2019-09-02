extern crate rand;

use rand::{thread_rng, Rng};
use vec3::{Vec3};

fn perlin_generate_perm() -> Vec<u32> {
  let mut p: Vec<u32> = (0..256).collect();
  let slice: &mut [u32] = &mut p;
  thread_rng().shuffle(slice);
  p.to_vec()
}

fn perlin_generate() -> Vec<f32> {
  let mut p = vec![];
  for _ in 0..256 {
    p.push(thread_rng().gen::<f32>())
  }
  p
}

fn ease_in_out(x: f32) -> f32 {
  x * x * (3. - 2. * x)
}

// function to interpolate values and make the noise smooth
fn trilinear_interp(c: [[[f32; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
  let mut acc = 0.0;
  for i_ in 0..2 {
    for j_ in 0..2 {
      for k_ in 0..2 {
        let i = i_ as f32;
        let j = j_ as f32;
        let k = k_ as f32;
        acc +=
          (i * u + (1. - i) * (1. - u)) *
          (j * v + (1. - j) * (1. - v)) *
          (k * w + (1. - k) * (1. - w)) * c[i_][j_][k_]
      }
    }
  }
  acc
}

pub struct Perlin {
  perm_x: Vec<u32>,
  perm_y: Vec<u32>,
  perm_z: Vec<u32>,
  ran_float: Vec<f32>
}

impl Perlin {
  pub fn new() -> Self {
    Perlin {
      perm_x: perlin_generate_perm(),
      perm_y: perlin_generate_perm(),
      perm_z: perlin_generate_perm(),
      ran_float: perlin_generate()
    }
  }

  pub fn noise(&self, p: Vec3) -> f32 {
    let u = ease_in_out(p.x - p.x.floor());
    let v = ease_in_out(p.y - p.y.floor());
    let w = ease_in_out(p.z - p.z.floor());
    let i = p.x.floor() as usize;
    let j = p.y.floor() as usize;
    let k = p.z.floor() as usize;
    
    let mut c: [[[f32; 2]; 2]; 2] = [
      [[0.0, 0.0], [0.0, 0.0]],
      [[0.0, 0.0], [0.0, 0.0]]
    ];

    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          let float_index: usize = (
            self.perm_x[(i + di) & 255] ^
            self.perm_y[(j + dj) & 255] ^
            self.perm_z[(k + dk) & 255]
          ) as usize;
          c[di][dj][dk] = self.ran_float[float_index];
        }
      }
    }

    return trilinear_interp(c, u, v, w);
  }
}