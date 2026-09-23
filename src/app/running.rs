//use crate::engine::simulation::Simulation;
use std::io::{self, Write};
/*
pub struct Running {
    simulation: Simulation,
}

impl Running {
    pub fn new(simulation: Simulation) -> Self {
        Running { simulation }
    }

    pub fn run(&mut self) {
        print!("test")
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
                //self.simulation.run();
            }

            "stop" => {
                println!("Simulation stopped!");
                //self.simulation.stop();
            }

            "exit" => {
                println!("Exiting...");
                break;
            }

            "" => {}

            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
    }
}*/
pub fn start_program() {
    // тут мы вызываем функцию run, которая использует данные из структуры Simulation
        
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
                //self.simulation.run();
            }

            "stop" => {
                println!("Simulation stopped!");
                //self.simulation.stop();
            }

            "exit" => {
                println!("Exiting...");
                break;
            }

            "" => {}

            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
    
}