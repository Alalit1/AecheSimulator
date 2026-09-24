use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Transform {
    pub fn new(position: [f32; 3], rotation: [f32; 3],scale: [f32; 3]) -> Self {
        Transform {
            position,
            rotation,
            scale,
        }
    }

}