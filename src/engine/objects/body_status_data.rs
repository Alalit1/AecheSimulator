use crate::engine::assets::resources::Resources;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyStatusData {
    pub resource: Resource,
    pub mass: f32,
    
}