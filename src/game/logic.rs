extern crate tobj;

use core::system;
use core::message::Message;
use core::bus::Bus;

use std::path::Path;
use std::collections::HashSet;

pub struct Logic {
    name: &'static str,
    status: system::Status,
    state: i32,
}

impl system::System for Logic {
    fn init() -> Self {
        Logic { name: "Logic", status: system::Status::Okay, state: 0 }
    }

    fn run(&mut self, bus: &mut Bus) -> &system::Status {

        match self.state {
            0 => {
                self.state += 1;
                let teapot = tobj::load_obj(&Path::new("resources/mesh/sphere_hipoly.obj"));
                assert!(teapot.is_ok());
                let (models, materials) = teapot.unwrap();
                let ref mesh = models.get(0).unwrap().mesh;
                println!("vert: {}", mesh.positions.len());
                println!("norm: {}", mesh.normals.len());
                println!("text: {}", mesh.texcoords.len());
                println!("indc: {}", mesh.indices.len());

                /*
                 * OpenGL only supports a single index. So we make the indices
                 * into tuples and then use a hashset to identify duplicates.
                 */
                let mut vec = Vec::new();
                for i in 0..mesh.indices.len() / 3 {
                    let index = i * 3;
                    let tuple = (mesh.indices.get(index),
                                 mesh.indices.get(index + 1),
                                 mesh.indices.get(index + 2));
                    vec.push(tuple);
                }
                let mut set = HashSet::new();
                let mut i = 0;
                let mut elements = Vec::new(); // temp for retaining index
                let mut element_array = Vec::new(); // actual element array
                let mut index_array = Vec::new(); // actual index array
                for index in vec.iter() {
                    if !set.contains(index) {
                        set.insert(index);
                        
                        elements.push(index); // push the unique element indeces
                        index_array.push(i); // push the index of this element
                        i = i+1;

                        let v = index.0.unwrap();
                        element_array.push(mesh.positions.get(*v as usize));
                        element_array.push(mesh.positions.get(*v as usize + 1));
                        element_array.push(mesh.positions.get(*v as usize + 2));

                        let t = index.1.unwrap();
                        element_array.push(mesh.texcoords.get(*t as usize));
                        element_array.push(mesh.texcoords.get(*t as usize + 1));

                        let n = index.2.unwrap();
                        element_array.push(mesh.normals.get(*n as usize));
                        element_array.push(mesh.normals.get(*n as usize + 1));
                        element_array.push(mesh.normals.get(*n as usize + 2));
                    }
                    else {
                        let pos = elements.iter().position(|&a| a == index).unwrap();
                        index_array.push(pos); // push index of non-unique element
                    }
                }
            },

            _ => {

            },
        }
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
