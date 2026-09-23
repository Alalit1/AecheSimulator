use crate::engine::assets::resources::Resources;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimulationData {
    pub resource: Resource,

    pub simulation_type: SimulationType,
}