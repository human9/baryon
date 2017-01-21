use std::rc::Rc;
use core::scene::Scene;

#[derive(Debug)]
pub enum Message {
    Shutdown,
    LoadScene(Rc<Scene>),
}
