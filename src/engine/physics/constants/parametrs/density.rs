use crate::engine::objects::physics::constants::parametr::Parametr;

pub struct Density {
    pub parametr: Parametr,
   
    pub value: f64,
}

impl Density {
    pub fn new() -> Self {
        Self {
            parametr: Parametr {
                id: 4,
                name: "Density".to_string(),
                
            },
            value,
        }
    }
}