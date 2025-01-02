use anyhow::Result;
use log::{error, info};
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
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("vulkeng")
            .with_inner_size(INITIAL_SIZE);

        match event_loop.create_window(window_attributes) {
            Ok(window) => self.window = Some(window),
            Err(e) => {
                error!("Could not create window: {}", e);
                event_loop.exit();
            }
        }
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
