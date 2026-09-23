use crate::engine::assets::resources::Resources;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CollisionShapeData {
    pub resource: Resource,
    pub shape: Shape,
    pub disabled: bool,
    
    pub debug_color: [f32; 4],
}