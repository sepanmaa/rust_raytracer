use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


macro_rules! v3 {
    ($x:expr, $y:expr, $z:expr) => { Vector3 { x: $x, y: $y, z: $z } }
}


impl Vector3 {
    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 { x: self.y*other.z-self.z*other.y,
                  y: self.z*other.x-self.x*other.z,
                  z: self.x*other.y-self.y*other.x }
    }

    pub fn dot(&self, other: Vector3) -> f64 {
        self.x*other.x+self.y*other.y+self.z*other.z
    }

    pub fn normalize(&self) -> Vector3 {
        let len = (self.x*self.x+self.y*self.y+self.z*self.z).sqrt();
        Vector3 { x: self.x / len, y: self.y / len, z: self.z / len }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.x * 255.0).min(255.0) as u8,
         (self.y * 255.0).min(255.0) as u8,
         (self.z * 255.0).min(255.0) as u8)
    }
}



impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x+other.x, y: self.y+other.y, z: self.z+other.z }
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 { x: self.x-other.x, y: self.y-other.y, z: self.z-other.z }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f64) -> Vector3 {
        Vector3 { x: self.x*scalar, y: self.y*scalar, z: self.z*scalar }
    }
}
