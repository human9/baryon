use core::system;
use core::message;

pub struct Simulation {
    name: &'static str,
    status: system::Status,
}

impl system::System for Simulation {
    fn init() -> Self {
        Simulation { name: "Simulation", status: system::Status::Okay }
    }

    fn run(&self) -> system::Status {
        self.status
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
