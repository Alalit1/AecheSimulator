struct Simulations<Body> {
     pub bodies: Vec<Body>,
     pub delta_time: f32,
     pub scene: Scene,
 }
 
 impl<Body> Simulations<Body> {
     pub fn new() -> Self {
         Simulations {
             bodies: Vec::new(),
             delta_time: 0.0,
             scene: Scene::new([0.0; 3]),
         }
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
     
     pub fn update(&mut self, delta_time: f32) {
         for body in &mut self.bodies {
             body.update(delta_time);
         }
     }
 }