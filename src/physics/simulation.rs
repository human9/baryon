use core::system;
use core::message::Message;
use core::bus::Bus;

pub struct Simulation {
    name: &'static str,
    status: system::Status,
}

impl system::System for Simulation {
    fn init() -> Self {
        Simulation { name: "Simulation", status: system::Status::Okay }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {
        &self.status
    }
    
    fn handle(&mut self, msg: &Message) {
        match msg {
            &Message::Shutdown => self.status = system::Status::Finished,
            _ => (),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
