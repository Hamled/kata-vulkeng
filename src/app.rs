use crate::engine::Engine;

use anyhow::Result;
use log::{debug, error, info};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::wayland::EventLoopBuilderExtWayland;
use winit::window::Window;

const INITIAL_SIZE: PhysicalSize<u32> = PhysicalSize::new(1920, 1080);

#[derive(Debug)]
enum UserEvent {
    SignalInterrupt,
}

#[derive(Default)]
struct App {
    window: Option<Window>,
    engine: Option<Engine>,
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("vulkeng")
            .with_inner_size(INITIAL_SIZE);

        let window = match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(e) => {
                error!("Could not create window: {}", e);
                event_loop.exit();
                return;
            }
        };
        debug!(
            "Created window with size {}x{}",
            window.inner_size().width,
            window.inner_size().height
        );

        let engine = match Engine::new(&window) {
            Ok(engine) => engine,
            Err(e) => {
                error!("Could not create engine: {}", e);
                event_loop.exit();
                return;
            }
        };

        self.window = Some(window);
        self.engine = Some(engine);
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::SignalInterrupt => event_loop.exit(),
        }
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        info!("Exiting application.");
    }
}

pub fn run() -> Result<()> {
    let mut app = App::default();

    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .with_wayland()
        .build()?;
    let proxy = event_loop.create_proxy();
    ctrlc::try_set_handler(move || {
        proxy
            .send_event(UserEvent::SignalInterrupt)
            .expect("App could not send SignalInterrupt.");
    })?;

    event_loop.run_app(&mut app)?;

    Ok(())
}
