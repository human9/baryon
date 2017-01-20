extern crate gl;

use core::system;
use core::message::Message;
use core::bus::Bus;

pub struct Rendering {
    name: &'static str,
    status: system::Status,
}

impl system::System for Rendering {
    fn init() -> Self {
        Rendering { name: "Rendering", status: system::Status::Okay }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        unsafe { 
            
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
        };


        &self.status
    }
    
    fn handle(&mut self, msg: &Message) {
        match msg {
            &Message::Shutdown => self.status = system::Status::Finished,
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
