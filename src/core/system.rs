use core::message;
use core::bus::Bus;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Status {
    Okay,
    Failed,
    Finished,
}

pub trait System {

    fn init() -> Self where Self:Sized;
    fn run(&mut self, &mut Bus) -> &Status;
    fn handle(&mut self, &message::Message);
    fn name(&self) -> &'static str;
}
