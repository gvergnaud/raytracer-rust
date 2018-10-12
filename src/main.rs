extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod hitable_list;
mod sphere;
mod material;

use rand::Rng;

use std::io::{self};
use std::sync::Arc;
use std::f64;

use vec3::{Vec3};
use ray::{Ray};
use hitable::{Hitable};
use hitable_list::{HitableList};
use sphere::{Sphere};
use material::{MaterialRecord, Lambertian, Metal};

fn background(r: &Ray) -> Vec3 {
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.);
    Vec3::new(1., 1., 1.) * (1. - t) + Vec3::new(0.5, 0.7, 1.) * t
}

fn color(r: &Ray, world: &HitableList, depth: u64) -> Vec3 {
    let t_min = 0.01;
    let t_max = f64::MAX;
    match world.hit(r, t_min, t_max) {
        Some(rec) => {
            match (depth < 50, (*rec.material).scatter(&r, &rec)) {
                (true, Some(mat_rec)) => {
                    mat_rec.attenuation * color(&mat_rec.scattered, &world, depth + 1)
                },
                _ => Vec3::fromf(0.),
            }
        },
        None => background(r),
    }
}

fn main() -> io::Result<()> {
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let lower_left_corner = Vec3::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical = Vec3::new(0., 2., 0.);
    let origin = Vec3::new(0., 0., 0.);

    println!("P3\n{} {}\n255", nx, ny);

    let world: HitableList = vec![
        Box::new(Sphere::new(
            Vec3::new(0., 0., -1.),
            0.5,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3))),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., -100.5 , -1.),
            100.0,
            Arc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.8))),
        )),
        Box::new(Sphere::new(
            Vec3::new(1., 0., -1.),
            0.5,
            Arc::new(
                Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.)
            ),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1., 0., -1.),
            0.5,
            Arc::new(
                Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.)
            ),
        )),
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::fromf(0.);

            for _ in 0..ns {
                let mut rng = rand::thread_rng();
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);

                let r = Ray {
                    origin: origin,
                    direction: lower_left_corner + u * horizontal + v * vertical,
                };

                col = col + color(&r, &world, 0);
            }

            let rgb = (col / (ns as f64)).sqrt() * 255.99;

            println!("{} {} {}", rgb.r() as u32, rgb.g() as u32, rgb.b() as u32);
        }
    }

    Ok(())
}
