use vec3::{Vec3};
use ray::{Ray};
use std::f64::consts::PI;

pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        angle: f64,
        ratio: f64,
    ) -> Self {
        let angle_rad = angle * PI / 180.;

        let half_height = (angle_rad / 2.).tan();
        let half_width = ratio * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * (half_width * 2.);
        let vertical = v * (half_height * 2.);
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction:
                self.lower_left_corner
                + self.horizontal * u
                + self.vertical * v
                - self.origin
        }
    }
}
