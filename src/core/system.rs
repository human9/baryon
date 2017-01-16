use core::message;

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Status {
    Empty,
    Full,
    Okay,
    Failed,
    Finished,
}

pub trait System {

    fn init() -> Self where Self:Sized;
    fn run(&mut self) -> Status;
    fn name(&self) -> &'static str;
}
