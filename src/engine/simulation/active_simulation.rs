use crate::engine::assets::resources::Resources;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct active_simulation {
    pub resource: Resource,

    pub simulation: Simulation,
}

/*
обробка даних симуляциї.
визиваючи у кожного обекта в сцене (симуляції) метод update, який обробляє дані симуляциї.
застосовуюци дані з сцени (SandBox)
*/

impl active_simulation {
    pub fn new(
        resource: Resource,
        simulation: Simulation,
    ) -> Self {
        Self {
            resource,
            simulation,
        }
    }
    pub fn update(&mut self, delta_time: f32) {
        // Update the simulation state based on the delta_time
        self.simulation.update(delta_time);
    }
}