use log::debug;
use thiserror::Error;
use winit::{dpi::PhysicalSize, window::Window};

#[derive(Error, Debug)]
pub enum EngineError {}

type Result<T> = std::result::Result<T, EngineError>;

pub struct Engine;

impl Engine {
    pub fn new(_window: &Window) -> Result<Self> {
        Ok(Self {})
    }

    pub fn destroy(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn render(&mut self, size: PhysicalSize<u32>) -> Result<()> {
        debug!("Rendering frame with size {}x{}", size.width, size.height);
        Ok(())
    }
}
