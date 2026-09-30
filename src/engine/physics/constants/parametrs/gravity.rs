use crate::engine::objects::physics::constants::parametr::Parametr;

pub struct Gravity {
    pub parametr: Parametr,
   
    pub value: f64,
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            parametr: Parametr {
                id: 1,
                name: "Gravity".to_string(),
            },
            value,
        }
    }
}