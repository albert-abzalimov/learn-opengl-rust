
use cgmath::{Point3, Vector3, vec3}; 
use super::{SCR_HEIGHT, SCR_WIDTH};

pub struct Camera{
    pub pos: Point3<f32>,
    pub front: Vector3<f32>,
    pub up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub sensitivity: f32,
    pub speed: f32,
    pub fov: f32,
    pub zoom: bool,
}

impl Camera{
    pub fn new() -> Self {
        Camera::default()
    }
}

impl Default for Camera{
    fn default() -> Self {
        Camera{
            pos: Point3::new(0.0, 0.0, 0.0),
            front: vec3(0.0, 0.0, -1.0),
            up: vec3(0.0, 1.0, 0.0),
            yaw: -90.0,
            pitch: 0.0,
            last_x: SCR_WIDTH as f32 / 2.,
            last_y: SCR_HEIGHT as f32 / 2.,
            sensitivity: 0.04,
            speed: 2.0,
            fov: 45.0,
            zoom: false,
        }
    } 
}