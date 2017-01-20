use std::collections::VecDeque;
use std::collections::LinkedList;
use core::message::Message;
use core::system::System;
use core::scene::Scene;

const INITIAL_SIZE: usize = 64;

pub struct Bus {
    ring: VecDeque<Message>,
    scene: LinkedList<Scene>,
}

impl Bus {
    pub fn new() -> Self {
        Bus {
             ring: VecDeque::with_capacity(INITIAL_SIZE),
             scene: LinkedList::new(),
        }
    }

    pub fn post(&mut self, msg: Message) {
        println!("{:.?} posted", msg);
        self.ring.push_back(msg);
    }

    pub fn deliver(&mut self, systems: &mut Vec<Box<System>>) {
        while let Some(msg) = self.ring.pop_front() {
            for system in systems.iter_mut() {
                system.handle(&msg);
            }
            println!("{:.?} delivered", msg);
        }
    }

    pub fn set_scene(&mut self, scene: Scene) {
        self.scene.clear();
        self.scene.push_front(scene); 
    }
    
    pub fn get_scene_mut(&mut self) -> &mut Scene {
        self.scene.front_mut().unwrap()
    }
    
    pub fn get_scene(&mut self) -> &Scene {
        self.scene.front().unwrap()
    }
}
