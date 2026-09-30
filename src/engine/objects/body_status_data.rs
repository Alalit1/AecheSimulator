use crate::engine::resources::resource::Resources;
use serde::{Serialize,Deserialize};


// ресурс який описует характиристики обекта (розмір  фагу состав обекта та інші)
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