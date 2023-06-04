use crate::vec::Vec3;

pub struct Camera {
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub position: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64, position: Vec3) -> Self {
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            viewport_height,
            viewport_width,
            position,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
}
