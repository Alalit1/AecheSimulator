use crate::engine::resources::resource::Resource;
use crate::engine::sand_box::environment::EnvironmentData;
use serde::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneData {
    pub resource: Resource,
    
    pub size: [f32; 3],
    pub environment: EnvironmentData,
}

impl SceneData {
    pub fn new(
        resource: Resource,
        size: [f32; 3],
        environment: EnvironmentData,
    ) -> Self {
        Self {
            resource,
            size,
            environment: EnvironmentData::new(),
        }
    }
}

