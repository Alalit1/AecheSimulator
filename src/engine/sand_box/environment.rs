use crate::engine::assets::resources::Resources;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnvironmentData {
    pub resource: Resource,

    pub gravity: f32,
    pub air_density: f32,
    pub drag_coefficient: f32,
}

impl EnvironmentData {
    pub fn new() -> Self {
        Self {
            resource: Resource::new(0, String::from("Default Environment")),
            gravity: 9.81,
            air_density: 1.225,
            drag_coefficient: 0.47,
        }
    }
}