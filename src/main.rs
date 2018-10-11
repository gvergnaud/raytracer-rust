mod vec3;
mod ray;
mod hitable;
mod hitable_list;
mod sphere;

use std::io::{self};
use vec3::{Vec3};
use ray::{Ray};
use hitable::{Hitable};
use hitable_list::{HitableList};
use sphere::{Sphere};

fn background(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    let t_min = 0.01;
    let t_max = 1000.;
    match world.hit(r, t_min, t_max) {
        Some(rec) => (1. + rec.normal) / 2.,
        None => background(r),
    }
}

fn main() -> io::Result<()> {
    let nx = 200;
    let ny = 100;
    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    println!("P3\n{} {}\n255", nx, ny);

    let world: HitableList = vec![
        Box::new(Sphere {
            center: Vec3::new(0., 0. , -2.),
            radius: 1.0,
        })
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = Ray {
                origin: origin,
                direction: lower_left_corner + u * horizontal + v * vertical,
            };

            let col = color(&r, &world);

            let rgb = col * 255.99;

            println!("{} {} {}", rgb.r() as u32, rgb.g() as u32, rgb.b() as u32);
        }
    }

    Ok(())
}
