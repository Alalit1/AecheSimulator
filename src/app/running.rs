use crate::engine::simulation::simulations::Simulations;
use crate::engine::simulation::simulation_data::SimulationData;
use crate::engine::resources::resource::Resource;
use crate::engine::sand_box::environment::EnvironmentData;
use crate::engine::sand_box::scene::Scene;
use crate::engine::sand_box::scene_data::SceneData;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::engine::objects::body::Body;
use crate::engine::objects::body_data::BodyData;
use crate::engine::objects::transforms::transform::Transform;
enum Command{
    Start,
    Stop,
    Exit,
}

pub fn start_program() {
    // тут мы вызываем функцию run, которая использует данные из структуры Simulation
    let mut simulation_data = SimulationData {
    resource: Resource::new(1,"Simulation".to_string()),
    sand_box: Scene::new(SceneData::new(Resource::new(2,"Simulation".to_string()), [10.0,10.0,10.0],  EnvironmentData::new())),
};
    simulation_data.sand_box.add_object(Body::new(BodyData::new(Resource::new(6,"Simulation".to_string()),Transform::new([0.0,50.0,0.0],[0.0,0.0,0.0],[0.0,0.0,0.0,])),[0.0, 1.0, 0.0]));
    let mut simulation = Simulations::new(simulation_data);
    let (tx, rx) = mpsc::channel::<Command>();

    // Потік симуляції
    let simulation_thread = thread::spawn(move || {
        let mut simulation = simulation;

         loop {
            // Перевіряємо, чи прийшла команда
            if let Ok(command) = rx.try_recv() {
                match command {
                    Command::Start => {
                        simulation.start();
                    }

                    Command::Stop => {
                        simulation.stop();
                    }

                    Command::Exit => {
                        break;
                    }
                }
            }

            simulation.update(0.1);

            thread::sleep(Duration::from_millis(16));
        }
    });

        loop {
      

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        let command = input.trim();

        match command {
            "help" => {
                println!("Available commands:");
                println!("  help");
                println!("  start");
                println!("  stop");
                println!("  exit");
            }

            "start" => {
                println!("Simulation started!");
                tx.send(Command::Start).unwrap();
            }

            "stop" => {
                println!("Simulation stopped!");
                tx.send(Command::Stop).unwrap();
            }

            "exit" => {
                println!("Exiting...");
                tx.send(Command::Exit).unwrap();
                break
            }

            "" => {}

            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
    simulation_thread.join().unwrap();
}