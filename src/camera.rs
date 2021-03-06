extern crate rand;
use self::rand::Rng;
use vec3::{Vec3};
use ray::{Ray};
use std::f32::consts::PI;

fn random_point_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let mut p: Vec3;
    loop {
        p = 2. * Vec3::new(rng.gen(), rng.gen(), 0.) - Vec3::new(1., 1., 0.);
        if p.dot(p) < 1.0 { break; }
    }
    p
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        up: Vec3,
        vertical_fov: f32,
        ratio: f32,
        aperture: f32,
        time0: f32,
        time1: f32,
    ) -> Self {
        let distance_to_focus = (look_at - look_from).length();
        let lens_radius = aperture / 2.;

        let angle_rad = vertical_fov * PI / 180.;

        let half_height = (angle_rad / 2.).tan();
        let half_width = ratio * half_height;

        let w = (look_from - look_at).unit_vector();
        let u = up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = u * (half_width * distance_to_focus * 2.);
        let vertical = v * (half_height * distance_to_focus * 2.);
        let depth = w * distance_to_focus;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - depth;

        Camera {
            u,
            v,
            lens_radius,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rand_origin = self.lens_radius * random_point_in_unit_disk();
        let offset = rand_origin.x * self.u + rand_origin.y * self.v;

        let ray_origin = self.origin + offset;

        let point_on_screen =
            self.lower_left_corner
            + self.horizontal * u
            + self.vertical * v;

        let mut rng = rand::thread_rng();
        let random_time = self.time0 + rng.gen::<f32>() * (self.time1 - self.time0);

        Ray {
            origin: ray_origin,
            direction: point_on_screen - ray_origin,
            time: random_time
        }
    }
}
