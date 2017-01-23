extern crate gl;

use core::system;
use core::message::Message;
use core::bus::Bus;
use core::scene::Scene;

use std::rc::Rc;
use std::option::Option;

pub struct Rendering {
    name: &'static str,
    status: system::Status,
    scene: Option<Rc<Scene>>,
}

impl Rendering {
    fn load_scene(&mut self, s: &Rc<Scene>) {
        self.scene = Some(s.clone());
    }

    fn shutdown(&mut self) {
        self.status = system::Status::Finished;
    }
}

impl system::System for Rendering {
    fn init() -> Self {
        Rendering { name: "Rendering", status: system::Status::Okay, scene: None }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        unsafe { 
            
            gl::Clear(gl::COLOR_BUFFER_BIT);
        
        };

        if let Some(ref scene) = self.scene {

        }

        &self.status
    }
    
    fn handle(&mut self, msg: &Message) {
        match msg {
            &Message::Shutdown => self.shutdown(),
            &Message::LoadScene(ref scene) => self.load_scene(scene),
            //_ => (),
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}
