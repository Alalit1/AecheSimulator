use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, DeviceId, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
struct WgpuState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,

    renderer: egui_wgpu::Renderer,
}

fn ui(ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("ArcheSimulator");

        ui.label("Physics Simulator");

        if ui.button("Start simulation").clicked() {
            println!("Start!");
        }

        if ui.button("Stop simulation").clicked() {
            println!("Stop!");
        }
    });
}

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,

    egui_ctx: egui::Context,

    egui_state: Option<egui_winit::State>,

    renderer: Option<egui_wgpu::Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("ArcheSimulator")
                )
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
    window_id: WindowId,
    event: WindowEvent,
) {
    let Some(window) = &self.window else {
        return;
    };

    if window.id() != window_id {
        return;
    }

    if let Some(state) = &mut self.egui_state {
        let response = state.on_window_event(
            window,
            &event,
        );

        if response.consumed {
            window.request_redraw();
        }
    }
    match event {
        WindowEvent::RedrawRequested => {
            println!("Window redraw requested");
            // 2. Отримуємо введення для egui
           
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