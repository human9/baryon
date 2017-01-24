extern crate glm;
use self::glm::builtin::fmod;
use self::glm::builtin::clamp;
use self::glm::Vector3;
use std::f32::consts::PI;

pub struct Camera {
    pub elevation: f32,
    pub azimuth: f32,
    pub radius: f32,
    pub position: glm::Vector3<f32>,
}

impl Camera {

    pub fn new() -> Self {
        Camera {
            elevation: 0.,
            azimuth: 0.,
            radius: 10.,
            position: Vector3::<f32>::new(0.,0.,0.),
        }
    }

    pub fn add_azimuth(&mut self, a: f32) {
        self.azimuth += a;
        self.azimuth = fmod(self.azimuth, 360.0);
        
    }

    pub fn add_elevation(&mut self, d: f32) {
        self.elevation += d;
        self.elevation = clamp(self.elevation, -89.9, 89.9);
    }

    pub fn add_forward(&mut self, z: f32) {
        let az = self.azimuth * PI / 180.;
        let el = self.elevation * PI / 180.;
        self.position[0] += z * el.cos() * -az.sin();
        self.position[1] += z * -el.sin();
        self.position[2] += z * el.cos() * az.cos();
    }
    
    pub fn add_strafe(&mut self, x: f32) {
        let az = self.azimuth * PI / 180.;
        self.position[0] += x * az.sin();
        self.position[2] += x * -az.cos();
    }

    // Return where the camera is
    pub fn get_eye(&self) -> Vector3<f32> {
        self.position
    }

    // Retern where the camera is looking
    pub fn get_center(&self) -> Vector3<f32> {
        let az = self.azimuth * PI / 180.;
        let el = self.elevation * PI / 180.;
        Vector3::<f32>::new(
            self.position[0] + self.radius * el.cos() * az.sin(),
            self.position[1] + self.radius * el.sin(),
            self.position[2] + -self.radius * el.cos() * az.cos(),
        )
    }

}
