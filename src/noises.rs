extern crate rand;

use rand::{thread_rng, Rng};
use vec3::{Vec3};

fn perlin_generate_perm() -> Vec<u32> {
  let mut p: Vec<u32> = (0..256).collect();
  let slice: &mut [u32] = &mut p;
  thread_rng().shuffle(slice);
  p.to_vec()
}

// array of random unit vectors
fn perlin_generate() -> Vec<Vec3> {
  let mut p = vec![];
  for _ in 0..256 {
    let rand_vec =
      Vec3::new(
        -1. + 2. * thread_rng().gen::<f32>(),
        -1. + 2. * thread_rng().gen::<f32>(),
        -1. + 2. * thread_rng().gen::<f32>(),
      ).unit_vector();
    p.push(rand_vec);
  }
  p
}

fn ease_in_out(x: f32) -> f32 {
  x * x * (3. - 2. * x)
}

// function to interpolate values and make the noise smooth
fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
  let uu = ease_in_out(u);
  let vv = ease_in_out(v);
  let ww = ease_in_out(w);
  let mut acc = 0.0;
  for i_usize in 0..2 {
    for j_usize in 0..2 {
      for k_usize in 0..2 {
        let i = i_usize as f32;
        let j = j_usize as f32;
        let k = k_usize as f32;
        let weight_v = Vec3::new(u - i, v - j, w - k);
        acc +=
          (i * uu + (1. - i) * (1. - uu)) *
          (j * vv + (1. - j) * (1. - vv)) *
          (k * ww + (1. - k) * (1. - ww)) *
          c[i_usize][j_usize][k_usize].dot(weight_v);
      }
    }
  }
  acc
}

pub struct Perlin {
  perm_x: Vec<u32>,
  perm_y: Vec<u32>,
  perm_z: Vec<u32>,
  ran_vec: Vec<Vec3>
}

impl Perlin {
  pub fn new() -> Self {
    Perlin {
      perm_x: perlin_generate_perm(),
      perm_y: perlin_generate_perm(),
      perm_z: perlin_generate_perm(),
      ran_vec: perlin_generate()
    }
  }

  pub fn noise(&self, p: Vec3) -> f32 {
    let u = p.x - p.x.floor();
    let v = p.y - p.y.floor();
    let w = p.z - p.z.floor();
    let i = p.x.floor() as i32;
    let j = p.y.floor() as i32;
    let k = p.z.floor() as i32;

    let mut c: [[[Vec3; 2]; 2]; 2] = [
      [
        [Vec3::fromf(0.), Vec3::fromf(0.)],
        [Vec3::fromf(0.), Vec3::fromf(0.)]
      ],
      [
        [Vec3::fromf(0.), Vec3::fromf(0.)],
        [Vec3::fromf(0.), Vec3::fromf(0.)]
      ]
    ];

    for di in 0..2 {
      for dj in 0..2 {
        for dk in 0..2 {
          // we have to convert to i32 because unsigned numbers (usize)
          // don't support negative values, and p.x.floor() can be negative
          // which results in a panic.
          let float_index: usize = (
            self.perm_x[((i + (di as i32)) & 255) as usize] ^
            self.perm_y[((j + (dj as i32)) & 255) as usize] ^
            self.perm_z[((k + (dk as i32)) & 255) as usize]
          ) as usize;
          c[di][dj][dk] = self.ran_vec[float_index];
        }
      }
    }

    return perlin_interp(c, u, v, w);
  }

  pub fn turb(&self, p: Vec3) -> f32 {
    let depth: i32 = 7;
    let mut tmp_p = p;
    let mut weight = 1.;
    (0..depth)
      .fold(0., |acc, _| {
        let res = acc + weight*self.noise(tmp_p);
        tmp_p *= 2.;
        weight *= 0.5;
        res
      })
      .abs()
  }
}
