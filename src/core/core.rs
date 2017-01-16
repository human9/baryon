use core::system::System;
use core::message::Message;
use core::bus::Bus;
use base::window::Windowing;
use physics::simulation::Simulation;

fn mainloop(mut systems: Vec<Box<System>>) {
    println!("We've entered the mainloop");

    let bus = Bus::new();
    //loop {
        for system in systems.iter_mut() {
            println!("{} status: {:?}", system.name(), system.run());
        }
    //}
}

pub fn startup() {

    let mut systems: Vec<Box<System>> = Vec::new();
    systems.push(Box::new(Windowing::init()));
    systems.push(Box::new(Simulation::init()));
    
    mainloop(systems);
}


