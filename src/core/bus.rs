use std::collections::VecDeque;
use core::message::Message;
use core::system::System;

const INITIAL_SIZE: usize = 64;

pub struct Bus {
    ring: VecDeque<Message>,
}

impl Bus {
    pub fn new() -> Self {
        Bus { ring: VecDeque::with_capacity(INITIAL_SIZE) }
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
}
