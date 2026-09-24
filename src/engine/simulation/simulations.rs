use serde::{Serialize,Deserialize};
use crate::engine::simulation::simulation_data::SimulationData;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Simulations {
    pub simulation_data: SimulationData,

    pub delta_time: f32,
    pub simulation_time: f32,
    pub running: bool,
 }
 
 impl Simulations {
     pub fn new(simulation_data: SimulationData) -> Self {
        Self {
            simulation_data,
            delta_time: 0.0,
            simulation_time: 0.0,
            running: false,
        }
     }
     pub fn run(&mut self) {
         // тут мы вызываем функцию run, которая использует данные из структуры Simulation
         loop {
             self.update(self.delta_time);
             
         }
     }
     pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

     
     pub fn update(&mut self, delta_time: f32) {

        if !self.running {
            return;
        }

        self.delta_time = delta_time;
        self.simulation_time += delta_time;
        self.simulation_data.sand_box.update(delta_time);

        println!("Time: {}", self.simulation_time);
    
     }
 }