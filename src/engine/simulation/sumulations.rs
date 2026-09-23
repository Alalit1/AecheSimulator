
use crate::engine::assets::resources::Resources;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Simulations<Body> {
    pub simulation_data: SimulationData,

    pub delta_time: f32,
    pub simulation_time: f32,
    pub is_running: bool,
 }
 
 impl<Body> Simulations<Body> {
     pub fn new() -> Self {
        Self {
            delta_time: 0.0,
            simulation_time: 0.0,
            is_running: false,
        }
     }
     pub fn run(&mut self) {
         // тут мы вызываем функцию run, которая использует данные из структуры Simulation
         loop {
             self.update(self.delta_time);
             self.print_parametrs();
         }
     }
     pub fn stop(&mut self) {
         // тут мы вызываем функцию stop, которая использует данные из структуры Simulation
     }
     pub fn add_body(&mut self, body: Body) {
         self.bodies.push(body);
     }
     pub fn remove_body(&mut self, index: usize) {
         if index < self.bodies.len() {
             self.bodies.remove(index);
         }
     }
     pub fn print_parametrs(&self) {
         for (i, body) in self.bodies.iter().enumerate() {
             println!("Body {}: Position: {:?}, Velocity: {:?}", i, body.position, body.velocity);
         }
     }
     pub fn print_sand_box_stats(&self) {
        println!("Sand Box Stats:");
         
     }
     pub fn print_stats_body(&self, index: usize) {
         if index < self.bodies.len() {
             let body = &self.bodies[index];
             println!("Body {}: Position: {:?}, Velocity: {:?}", index, body.position, body.velocity);
         } else {
             println!("Invalid body index: {}", index);
         }
     }
     
     pub fn update(&mut self, delta_time: f32) {

        if !self.is_running {
            return;
        }

        self.delta_time = delta_time;
        self.simulation_time += delta_time;

        println!("Time: {}", self.simulation_time);
    
     }
 }