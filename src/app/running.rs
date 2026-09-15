use engine::simulation::Simulation;

pub struct Running {
    simulation: Simulation,
}

impl Running {
    pub fn new(simulation: Simulation) -> Self {
        Running { simulation }
    }

    pub fn run(&mut self) {
        // тут мы вызываем функцию run, которая использует данные из структуры Simulation
        self.simulation.run();
    }
}