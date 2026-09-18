use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};



#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        self.window = Some(window);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
) {
    match event {
        WindowEvent::RedrawRequested => {
            println!("Window redraw requested");
        }

        WindowEvent::CloseRequested => {
            event_loop.exit();
        }

        _ => {}
    }
}

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                println!("Mouse moved by {:?}", delta);
            }

            DeviceEvent::Button { button, state } => {
                println!(
                    "Mouse button {} is {:?}",
                    button,
                    state
                );
            }

            DeviceEvent::Key(key) => {
                println!("Key event: {:?}", key);
            }

            _ => {}
        }
    }
}

pub fn mains() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    event_loop.run_app(&mut app).unwrap();

    
}