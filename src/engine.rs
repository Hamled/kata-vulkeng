use thiserror::Error;
use winit::window::Window;

#[derive(Error, Debug)]
pub enum EngineError {}

pub struct Engine;

impl Engine {
    pub fn new(_window: &Window) -> Result<Self, EngineError> {
        Ok(Self {})
    }
}
