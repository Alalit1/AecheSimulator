use crate::engine::assets::resources::Resources;

pub struct BodyData {
    pub id: u64,
    pub name: String,

    pub resource: Resource,
    pub environment: EnvironmentData,
}

    pub position: [f32; 3] = [0.0, 0.0, 0.0],
    pub rotation: [f32; 4] = [0.0, 0.0, 0.0, 1.0],
    pub scale: [f32; 3] = [1.0, 1.0, 1.0],
    pub mass: f32 = 1.0,
