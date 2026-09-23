use serde::{Deserialize, Serialize};

use crate::engine::assets::resources::Resources;
use super::shape::Shape;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SphereShape {
    pub resource: Resource,

    pub radius: f32,
}

impl Shape for SphereShape {
    fn volume(&self) -> f32 {
        (4.0 / 3.0)
            * std::f32::consts::PI
            * self.radius.powi(3)
    }

    fn surface_area(&self) -> f32 {
        4.0
            * std::f32::consts::PI
            * self.radius.powi(2)
    }
}