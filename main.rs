use std::io::{self};

fn main() -> io::Result<()> {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;

            println!("{} {} {}", (r * 255.99) as u32, (g * 255.99) as u32, (b * 255.99) as u32);
        }
    }

    Ok(())
}
