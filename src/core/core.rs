use core::system::System;
use base::window::Windowing;
use physics::simulation::Simulation;

pub fn startup() {
    
    let mut systems: Vec<Box<System>> = Vec::new();
    systems.push(Box::new(Windowing::init()));
    systems.push(Box::new(Simulation::init()));
    
    mainloop(systems);
}

fn mainloop(systems: Vec<Box<System>>) {
    println!("We've entered the mainloop");
    loop {
        for system in systems.iter() {
            println!("{} status: {:?}", system.name(), system.run());
        }
    }
}
