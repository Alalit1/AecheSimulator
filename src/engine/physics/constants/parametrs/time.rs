use crate::engine::objects::physics::constants::parametr::Parametr;

pub struct Time {
    pub parametr: Parametr,
   
    pub value: f64,
}

impl Time {
    pub fn new() -> Self {
        Self {
            parametr: Parametr {
                id: 2,
                name: "Time".to_string(),
                
            },
            value,
        }
    }
}