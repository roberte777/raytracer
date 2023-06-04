use raytracer::{camera::Camera, image::FileType, renderer::Renderer, vec::Vec3};

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const ASPECT_RATIO: f64 = 16. / 9.;
fn main() {
    let c = Camera::new(2., ASPECT_RATIO, 1., Vec3::new(0., 0., 0.));
    let image = Renderer::render(WIDTH, HEIGHT, &c);
    image.write_to_file("image.ppm", FileType::PPM);
}
