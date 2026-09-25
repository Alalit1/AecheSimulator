use crate::engine::objects::body::Body; 
use crate::engine::objects::body_data::BodyData; 

// Body manager

pub struct BodyManager{
    bodies: Vec<Body>,
}
//

impl BpubodyManager{
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
        }
    }
    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
//-> Result<(),>
    pub fn remove_body(&mut self, index: usize) {
        self.bodies.remove(index).expect("not body");
    }

    pub fn update(&mut self, delta_time: f32) {
        for body in &mut self.bodies {
            body.update(delta_time);
        }
    }

    pub fn get_array(&mut self) -> Vec<Body>{
        return self.bosies
    }

    pub fn get_parametrs_body(&mut self, index: usize) -> Result<&BodyData, String> {
        let body = self.bodies
            .get(index)
            .ok_or("Error: body not found")?;

        Ok(&body.body_data)
            
    }
    pub fn set_parametrs_body(&mut self, index: usize, body_data:BodyData){
        self.bodies[index].body_data = body_data 
    }
}
