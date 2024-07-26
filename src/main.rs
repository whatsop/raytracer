// pull mod declarations
mod vec3;

use vec3::Vec3;

fn main() {
    println!("Hello, world!");

    let vector_a = Vec3::from(1.0, 1.0, 1.0);
    let vector_b = Vec3::from(1.0, 2.0, 0.0);

    let sum = vector_a+vector_b;

    println!("{:?}", sum);
    println!("{:?}", vector_a);
}
