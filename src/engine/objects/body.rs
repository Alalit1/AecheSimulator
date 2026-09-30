use crate::engine::objects::body_data::BodyData;
use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Body {
    pub body_data: BodyData,
    pub velocity: [f32; 3],

}

impl Body {
    pub fn new(body_data: BodyData, _velocity: [f32; 3]) -> Self {
        Body {
            body_data,
            velocity: [0.0, 1.0, 0.0],
        }
    }
    // замінити переписати
    pub fn update(&mut self, delta_time: f32) {
 
        //0 * delta_time + 9.81 * delta_time.powi(2) / 2.0
        self.body_data.physics_model.update(self.body_data.body_status_data,time)
        /*self.body_data.transform.position[0] += self.velocity[0] * delta_time;
        self.body_data.transform.position[1] += self.velocity[1] * delta_time;
        self.body_data.transform.position[2] += self.velocity[2] * delta_time;*/
    }
}

trait Updatable {
    fn update(&mut self, delta_time: f32);
}