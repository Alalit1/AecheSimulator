use crate::engine::resources::resource::Resources;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyStatusData {
    pub resource: Resource,
    pub mass: f32,
    
    
}