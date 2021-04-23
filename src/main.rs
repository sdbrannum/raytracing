use std::fs;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut data: String = "P3\n".to_owned();
    data.push_str(format!("{} {}\n255\n", nx, ny).as_str());

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99 * r) as i32;
            let ig = (255.99 * g) as i32;
            let ib = (255.99 * b) as i32;
            data.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }
    fs::write("test.ppm", data).expect("unable to write file");
}
