mod engine;
mod app;
use engine::simulation::simulations::Simulations;
use app::running::Running;

fn main() {
    Running::new(
        Simulations::new()
    ).run();
}