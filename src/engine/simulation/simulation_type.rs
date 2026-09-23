use crate::engine::assets::resources::Resources;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimulationType {
    pub name: String,
    pub description: String,
}