use std::array::from_fn;

use glam::Vec3;

const FUEL_DENSITY: f32 = 4.0;

pub struct FourWheelVehicle {
    wheels: [Wheel; 4],
    // todo: collider
    /// the dry weight of the vehicle
    weight: f32,
    fuel: f32,
    position: Vec3,
    speed: Vec3,
}
impl FourWheelVehicle {
    pub fn new(weight: f32, fuel: f32, wheels: [Wheel; 4], position: Vec3, speed: Vec3) -> Self {
        Self {
            wheels,
            weight,
            fuel,
            position,
            speed,
        }
    }
    pub fn get_wheel_positions(&self) -> [Vec3; 4] {
        from_fn(|i| {
            self.wheels[i].pos + self.position
        })
    }
    pub fn get_vehicle_position(&self) -> Vec3 {
        self.position
    }
    pub fn get_speed(&self) -> Vec3 {
        self.speed
    }
    pub fn get_weight(&self) -> f32 {
        self.weight + self.fuel * FUEL_DENSITY
    }
}
pub struct Wheel {
    pub rest_length: f32,
    pub suspension_range: f32,
    pub pos: Vec3,
    /// in radians per second
    pub angular_speed: f32,
    pub radius: f32,
    pub grip: f32,
}
impl Wheel {
    pub fn new(pos: Vec3, radius: f32, grip: f32, rest_length: f32, suspension_range: f32) -> Self {
        Self { pos, radius, grip, angular_speed: 0.0, rest_length, suspension_range}
    }
}