use std::cmp;
use std::ops::{Add, Div, Mul, Neg, Sub, Index, MulAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x,
            y,
            z,
        }
    }

    pub fn fromf(v: f32) -> Vec3 {
        Vec3 {
            x: v,
            y: v,
            z: v,
        }
    }

    pub fn zero() -> Vec3 {
        Vec3 { x: 0., y: 0., z: 0. }
    }

    pub fn one() -> Vec3 {
        Vec3 { x: 1., y: 1., z: 1. }
    }

    pub fn r(self) -> f32 {
        self.x
    }

    pub fn g(self) -> f32 {
        self.y
    }

    pub fn b(self) -> f32 {
        self.z
    }

    pub fn length(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn squared_length(self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    pub fn cross(self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: -(self.x * b.z - self.z * b.x),
            z: self.x * b.y - self.y * b.x
        }
    }

    pub fn unit_vector(self) -> Vec3 {
        let l = self.length();
        self / (Vec3 { x: l, y: l, z: l })
    }

    pub fn sqrt(self) -> Vec3 {
        Vec3::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<Vec3> for f32 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

 impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        Vec3 {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Sub<Vec3> for f32 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl<'a> Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: &'a Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl cmp::PartialEq for Vec3 {
    fn eq(&self, v: &Vec3) -> bool {
        self.x == v.x && self.y == v.y && self.z == v.z
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("You can't access an index greater than 2 on a Vec3.")
        }
    }
}
