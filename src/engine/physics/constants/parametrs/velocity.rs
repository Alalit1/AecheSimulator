use crate::engine::objects::physics::constants::parametr::Parametr;

pub struct Velocity {
    pub parametr: Parametr,
   
    pub value: f64,
}

impl Velocity {
    pub fn new() -> Self {
        Self {
            parametr: Parametr {
                id: 3,
                name: "Velocity".to_string(),
                
            },
            value,
        }
    }
}