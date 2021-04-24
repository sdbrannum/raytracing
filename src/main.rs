mod ray;
mod vec3;

use ray::Ray;
use std::fs;
use vec3::Vec3;

fn main() {
    let nx = 200;
    let ny = 100;
    let mut data: String = "P3\n".to_owned();
    data.push_str(format!("{} {}\n255\n", nx, ny).as_str());

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = i as f32 / ny as f32;
            let ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let color = color(ray);

            let ir = (255.99 * color.r()) as i32;
            let ig = (255.99 * color.g()) as i32;
            let ib = (255.99 * color.b()) as i32;

            data.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }
    fs::write("test.ppm", data).expect("unable to write file");
}

fn color(r: Ray) -> Vec3 {
    let unit_direction = Vec3::unit_vector(*r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t;
}
