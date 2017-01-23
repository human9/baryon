extern crate tobj;

use core::system;
use core::object;
use core::scene::Scene;
use core::message::Message;
use core::bus::Bus;
use render::shader;
use std::collections::LinkedList;

use std::path::Path;
use std::rc::Rc;

pub struct Logic {
    name: &'static str,
    status: system::Status,
    state: i32,
}

impl system::System for Logic {
    fn init() -> Self {
        Logic { name: "Logic", status: system::Status::Okay, state: 0, }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        match self.state {
            0 => {
                self.state += 1;

                let teapot_raw = tobj::load_obj(&Path::new("resources/mesh/sphere_hipoly.obj"));
                assert!(teapot_raw.is_ok());
                let models  = teapot_raw.unwrap().0;
                let teapot = object::tobj_to_object(&models.get(0).unwrap());

                let shader;
                unsafe { shader = shader::glsl_init("resources/glsl/standard.vert", "resources/glsl/standard.frag"); };
                let rc_shader: Rc<shader::Shader> = Rc::new(shader);
                
                let mut scene = Scene { objects: LinkedList::new(), name: "test scene" };
                scene.objects.push_back( (teapot, rc_shader.clone()) );
                let rc_scene: Rc<Scene> = Rc::new(scene);
                bus.post(Message::LoadScene(rc_scene));
            },

            _ => {

            },
        }
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
