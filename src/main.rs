use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

// pull mod declarations
mod color;
mod ray;
mod vec3;

use color::Color;
use ray::Ray;
use vec3::Vec3;

fn main() {
    // image settings
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: f32 = 400.0;
    let mut image_height: f32 = image_width / aspect_ratio;
    image_height = if image_height < 1.0 {
        1.0
    } else {
        image_height
    };
    let image_path = Path::new("render/image.ppm");

    // render
    render(image_width, image_height, &image_path);
}

/// render the image
fn render(image_width: f32, image_height: f32, image_path: &Path) {
    // get image file
    let mut file = File::create(image_path).expect("Path is incorrect !");

    // write to the file
    let main_data = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(main_data.as_bytes())
        .expect("Could not write to the file !");

    // camera and viewport settings
    let focal_length: f32 = 1.0;
    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * (image_width / image_height);
    let camera_center = Vec3::new(0.0, 0.0, 0.0);

    // calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    // calculate the location of the upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // pixels are written out in rows
    // every row of pixels is written out left to right, from top to bottom
    let mut pixel_count = 0;
    for row in 0..image_height as i32 {
        for column in 0..image_width as i32 {
            // let red = column as f32 / (image_width - 1.0);
            // let green = row as f32 / (image_height - 1.0);
            // let blue = 0.0f32;

            let pixel_center =
                pixel00_location + (pixel_delta_u * column as f32) + (pixel_delta_v * row as f32);
            let ray = Ray::new(camera_center, pixel_center - camera_center);

            // write rgb data to the file
            let color_bytes = ray_color(&ray).as_bytes();
            let rgb_data = format!("{} {} {}\n", color_bytes[0], color_bytes[1], color_bytes[2]);
            println!("{}", rgb_data);
            file.write(rgb_data.as_bytes())
                .expect("Could not write to the file !");

            // print progress
            pixel_count += 1;
            println!(
                "{} pixel rendered / {} pixels left\n",
                pixel_count,
                image_height * image_width
            );
        }
    }
}

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = *center - ray.origin;
    let a = Vec3::dot(&ray.direction, &ray.direction);
    let b = -2.0 * Vec3::dot(&ray.direction, &oc);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant >= 0.0
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        Color::new(1.0, 0.0, 0.0)
    } else {
        Color::new(1.0, 1.0, 1.0)
        // let unit_direction = ray.direction.normalize();
        // let a = 0.5 * (unit_direction.y + 1.0);
        // (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
