use std::io::{self, Write};

use crate::{
    camera::Camera,
    image::{Image, Pixel},
    ray::Ray,
    vec::Vec3,
};

pub struct Renderer {}

impl Renderer {
    pub fn render(width: usize, height: usize, camera: &Camera) -> Image {
        let mut pixels: Vec<Pixel> = Vec::new();
        for j in (0..height).rev() {
            for i in 0..width {
                // print lines remaining
                print!("\rLines remaining: {}", j);
                io::stdout().flush().unwrap();
                let u = i as f64 / width as f64;
                let v = j as f64 / height as f64;
                let r = Ray::new(
                    camera.position,
                    camera.lower_left_corner + u * camera.horizontal + v * camera.vertical
                        - camera.position,
                );
                let color = Renderer::ray_color(&r);
                pixels.push(Pixel::from_vec3(&color));
            }
        }
        Image::new(width, height, pixels)
    }
    pub fn ray_color(ray: &Ray) -> Vec3 {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
