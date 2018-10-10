use std::io::{self};
mod vec3;

fn main() -> io::Result<()> {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let v = vec3::Vec3 {
                x: (i as f64) / (nx as f64),
                y: (j as f64) / (ny as f64),
                z: 0.2,
            };

            let rgb = v * vec3::fromf(255.99);

            println!("{} {} {}", rgb.r() as u32, rgb.g() as u32, rgb.b() as u32);
        }
    }

    Ok(())
}
