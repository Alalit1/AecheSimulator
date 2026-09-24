use serde::{Serialize,Deserialize};
use crate::engine::resources::resource::Resource;
use crate::engine::sand_box::scene::Scene;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimulationData {
    pub resource: Resource,
    // обети симуляциї 
    pub sand_box: Scene,
    // скорость симуляциї та інші позначення для симуляциї
    //pub parameters: SimulationParameters,
}