use vec3::{Vec3};

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    fn point_at_parameter(self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}
