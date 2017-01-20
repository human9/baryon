use core::system::{System, Status};
use core::bus::Bus;
use base::window::Windowing;
use physics::simulation::Simulation;
use render::render::Rendering;
use game::logic::Logic;

use std::time::{Instant, Duration};
use std::thread;

pub fn mainloop() {
    println!("We've entered the mainloop");

    let mut bus = Bus::new();
    let mut systems = vec![
        Box::new(Windowing::init()) as Box<System>,
        Box::new(Simulation::init()),
        Box::new(Rendering::init()),
        Box::new(Logic::init()),
    ];

    let mut complete: usize;
    let mut instant: Instant;
    let mut duration: Duration;
    let mut diff: u64;

    loop {

        complete = 0;
        instant = Instant::now();

        /*
         * Run each system in turn, passing a reference to the message bus.
         */
        for system in systems.iter_mut() {
            
            match system.run(&mut bus) {
                &Status::Okay => (),
                &Status::Failed => println!("{} has failed", system.name()),
                &Status::Finished => complete += 1,
            }
        }

        if complete == systems.len() {
            break;
        }

        /*
         * Deliver any posted messages to all systems.
         */
        bus.deliver(&mut systems);


        /*
         * Prevent wasting cpu time.
         */
        duration = instant.elapsed();
        diff = (duration.as_secs() * 1_000) + (duration.subsec_nanos() / 1_000_000) as u64;
        if diff < 10 {
            thread::sleep(Duration::from_millis(10));
        }
    }
}
