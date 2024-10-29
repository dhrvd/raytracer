mod vec3;

use vec3::Vec3;

const IMAGE_HEIGHT: i32 = 256;
const IMAGE_WIDTH: i32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let color = Vec3::new(
                i as f32 / (IMAGE_WIDTH - 1) as f32,
                j as f32 / (IMAGE_HEIGHT - 1) as f32,
                0_f32,
            );

            // translate the [0, 1] component values to the byte range [0, 255]
            let rbyte = (255.999 * color.x) as i32;
            let gbyte = (255.999 * color.y) as i32;
            let bbyte = (255.999 * color.z) as i32;

            println!("{} {} {}", rbyte, gbyte, bbyte);
        }
    }
}
