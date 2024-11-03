use crate::math::Vec3;

pub trait Texture {
    fn value(&self, uv: (f32, f32), point: &Vec3) -> Vec3;
}

pub struct Solid {
    albedo: Vec3,
}

impl Solid {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Texture for Solid {
    fn value(&self, _uv: (f32, f32), _point: &Vec3) -> Vec3 {
        self.albedo
    }
}

pub struct Checkered {
    inv_scale: f32,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Checkered {
    pub fn new(scale: f32, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }
}

impl Texture for Checkered {
    fn value(&self, uv: (f32, f32), point: &Vec3) -> Vec3 {
        let x_integer = (self.inv_scale * point.x).floor() as i32;
        let y_integer = (self.inv_scale * point.y).floor() as i32;
        let z_integer = (self.inv_scale * point.z).floor() as i32;

        if (x_integer + y_integer + z_integer) % 2 == 0 {
            self.even.value(uv, point)
        } else {
            self.odd.value(uv, point)
        }
    }
}
