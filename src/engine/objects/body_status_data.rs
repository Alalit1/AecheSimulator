use crate::engine::resources::resource::Resources;
use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyStatusData {
    pub resource: Resource,
    
    pub materisls: Materials,
    //pub volume: f32,
    // маса
    pub mass: f32,
    // вага
    pub weifht: f32,
    
}