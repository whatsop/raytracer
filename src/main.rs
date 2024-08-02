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
    let image_width: f32 = 2.0;
    let image_height: f32 = 2.0;
    let image_path = Path::new("render/image.ppm");

    // render
    render(image_width, image_height, &image_path);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let direction = Vec3::new(1.0, 0.0, 0.0);
    let ray_a = Ray::new(origin, direction);
    println!("{:?}", ray_a);
    println!("{:?}", ray_a.at(2.0));

}

/// render the image
fn render(image_width: f32, image_height: f32, image_path: &Path) {

    // get image file
    let mut file = File::create(image_path).expect("Path is incorrect !");

    // write to the file
    let main_data = format!("P3\n{} {}\n255\n", image_width, image_height);
    file.write(main_data.as_bytes())
        .expect("Could not write to the file !");

    // pixels are written out in rows
    // every row of pixels is written out left to right, from top to bottom
    let mut pixel_count = 0;
    for row in 0..image_height as i32 {
        for column in 0..image_width as i32 {
            let red = column as f32 / (image_width - 1.0);
            let green = row as f32 / (image_height - 1.0);
            let blue = 0.0f32;

            // write rgb data to the file
            let color_bytes = Color::new(red, green, blue).as_bytes();
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
