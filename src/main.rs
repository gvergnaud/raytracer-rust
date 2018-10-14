extern crate rand;

mod vec3;
mod ray;
mod hitable;
mod material;
mod camera;

use rand::Rng;

use std::io::{self};
use std::sync::Arc;
use std::f64;

use vec3::{Vec3};
use ray::{Ray};
use hitable::{Hitable, HitableList, Sphere};
use material::{Lambertian, Metal, Dielectric};
use camera::{Camera};

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

fn create_world() -> HitableList {
    let beige = Vec3::new(246., 211., 195.) / 255.;
    let brown = Vec3::new(163., 82., 51.) / 255.;

    let colors = [
        Vec3::new(139., 75., 98.) / 255.,
        Vec3::new(187., 111., 107.) / 255.,
        Vec3::new(234., 150., 116.) / 255.,
        Vec3::new(252., 188., 128.) / 255.,
        Vec3::new(247., 226., 156.) / 255.,
        beige,
        brown,
    ];

    let mut world : HitableList;

    let intersects_with_main_spheres = |new_center: Vec3, new_radius: f64| {
        [
            (Vec3::new(0., 0., -1.), 0.7),
            (Vec3::new(-1., 0., -1.), 0.7),
            (Vec3::new(1., 0., -1.), 0.7),
        ].iter().fold(false, |acc, (center, radius)| {
            if acc { return acc };
            let distance = (new_center - center).length();
            distance < new_radius || distance < *radius
        })
    };

    let random_color_and_position = || {
        let mut rng = rand::thread_rng();
        let mut x : f64;
        let mut z : f64;
        loop {
            x = rng.gen_range::<f64>(-5., 5.);
            z = rng.gen_range::<f64>(-5., 5.);
            if !intersects_with_main_spheres(Vec3::new(x, -0.3, z), 0.2) { break; }
        }

        (
            colors[rng.gen_range::<u64>(0, colors.len() as u64) as usize],
            x,
            z,
        )
    };

    world = vec![
        Box::new(Sphere::new(
            Vec3::new(0., -100.5 , -1.),
            100.0,
            Arc::new(
                Lambertian::new(Vec3::new(0.8, 0.8, 0.8))
            ),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., 0., -1.),
            0.5,
            Arc::new(
                Lambertian::new(Vec3::new(139., 75., 98.) / 255.)
            ),
        )),
        Box::new(Sphere::new(
            Vec3::new(-1., 0., -1.),
            0.5,
            Arc::new(
                Dielectric::new(1.5)
            ),
        )),
        Box::new(Sphere::new(
            Vec3::new(1., 0., -1.),
            0.5,
            Arc::new(
                Metal::new(beige, 0.2)
            ),
        )),
    ];

    for _ in 0..50 {
        let (color, x, z) = random_color_and_position();
        world.push(Box::new(
            Sphere::new(
                Vec3::new(x, -0.3, z),
                0.2,
                Arc::new(
                    Lambertian::new(color)
                )
            )
        ));
    }

    for _ in 0..25 {
        let (color, x, z) = random_color_and_position();
        let fuzz = rand::thread_rng().gen::<f64>();
        world.push(Box::new(
            Sphere::new(
                Vec3::new(x, -0.3, z),
                0.2,
                Arc::new(
                    Metal::new(color, fuzz)
                )
            )
        ));
    }

    for _ in 0..15 {
        let (_, x, z) = random_color_and_position();
        world.push(Box::new(
            Sphere::new(
                Vec3::new(x, -0.3, z),
                0.2,
                Arc::new(
                    Dielectric::new(1.5)
                )
            )
        ));
    }

    world
}

fn main() -> io::Result<()> {
    let nx = 600;
    let ny = 400;
    let ns = 150;

    println!("P3\n{} {}\n255", nx, ny);

    let world = create_world();

    let camera = Camera::new(
        Vec3::new(2.5, 0.5, 0.5),
        Vec3::new(0., -0.2, -1.),
        Vec3::new(0., 1., 0.),
        35.,
        (nx as f64) / (ny as f64),
        0.05
    );

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::fromf(0.);

            for _ in 0..ns {
                let mut rng = rand::thread_rng();
                let u = ((i as f64) + rng.gen::<f64>()) / (nx as f64);
                let v = ((j as f64) + rng.gen::<f64>()) / (ny as f64);

                let r = camera.get_ray(u, v);

                col = col + color(&r, &world, 0);
            }

            let rgb = (col / (ns as f64)).sqrt() * 255.99;

            println!("{} {} {}", rgb.r() as u32, rgb.g() as u32, rgb.b() as u32);
        }
    }

    Ok(())
}
