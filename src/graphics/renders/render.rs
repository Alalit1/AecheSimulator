use std::sync::Arc;
use winit::{
    raw_window_handle::HasDisplayHandle,
    window::Window,
};
pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub window: Arc<Window>,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        // ---------------------------------------------
        // 1. WGPU Instance
        // ---------------------------------------------

        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                flags: wgpu::InstanceFlags::default(),
                memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
                backend_options: wgpu::BackendOptions::default(),

                // wgpu 30
                display: wgpu::DisplayHandle::default(),
            },
        );

        // ---------------------------------------------
        // 2. Surface
        // ---------------------------------------------

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        // ---------------------------------------------
        // 3. Adapter
        // ---------------------------------------------

        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface: Some(&surface),
                    force_fallback_adapter: false,

                    // wgpu 30
                    apply_limit_buckets: false,
                },
            )
            .await
            .expect("Failed to find suitable GPU adapter");

        println!("GPU: {}", adapter.get_info().name);

        // ---------------------------------------------
        // 4. Device + Queue
        // ---------------------------------------------

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("ArcheSimulator Device"),

                    required_features: wgpu::Features::empty(),

                    required_limits: wgpu::Limits::default(),

                    memory_hints: wgpu::MemoryHints::Performance,

                    // wgpu 30
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),

                    trace: wgpu::Trace::Off,
                },
            )
            .await
            .expect("Failed to create device");

        // ---------------------------------------------
        // 5. Surface capabilities
        // ---------------------------------------------

        let capabilities = surface.get_capabilities(&adapter);

        // ---------------------------------------------
        // 6. Surface format
        // ---------------------------------------------

        let format = capabilities
            .formats
            .iter()
            .copied()
            .find(|format| format.is_srgb())
            .unwrap_or(capabilities.formats[0]);

        // ---------------------------------------------
        // 7. Window size
        // ---------------------------------------------

        let size = window.inner_size();

        // ---------------------------------------------
        // 8. Surface configuration
        // ---------------------------------------------

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,

            format,

            width: size.width,
            height: size.height,

            present_mode: wgpu::PresentMode::Fifo,

            desired_maximum_frame_latency: 2,

            alpha_mode: capabilities.alpha_modes[0],

            view_formats: vec![],

            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        // ---------------------------------------------
        // 9. Configure Surface
        // ---------------------------------------------

        surface.configure(&device, &config);

        // ---------------------------------------------
        // 10. Renderer
        // ---------------------------------------------

        Self {
            surface,
            device,
            queue,
            config,
            window,
        }
    }

    // =================================================
    // RENDER
    // =================================================

    pub fn render(&mut self) {
        // ---------------------------------------------
        // 1. Отримуємо SurfaceTexture
        // ---------------------------------------------

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                output
            }

            wgpu::CurrentSurfaceTexture::Timeout => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Occluded => {
                return;
            }

            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(
                    &self.device,
                    &self.config,
                );

                return;
            }

            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(
                    &self.device,
                    &self.config,
                );

                return;
            }

            wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        // ---------------------------------------------
        // 2. Створюємо TextureView
        // ---------------------------------------------

        let view = output
            .texture
            .create_view(
                &wgpu::TextureViewDescriptor::default()
            );

        // ---------------------------------------------
        // 3. Command Encoder
        // ---------------------------------------------

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            },
        );

        // ---------------------------------------------
        // 4. Render Pass
        // ---------------------------------------------

        {
            let _render_pass =
                encoder.begin_render_pass(
                    &wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),

                        color_attachments: &[Some(
                            wgpu::RenderPassColorAttachment {
                                view: &view,

                                depth_slice: None,

                                resolve_target: None,

                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(
                                        wgpu::Color {
                                            r: 0.1,
                                            g: 0.2,
                                            b: 0.3,
                                            a: 1.0,
                                        },
                                    ),

                                    store: wgpu::StoreOp::Store,
                                },
                            },
                        )],

                        depth_stencil_attachment: None,

                        occlusion_query_set: None,

                        timestamp_writes: None,

                        multiview_mask: None,
                    },
                );
        }

        // ---------------------------------------------
        // 5. Відправляємо команди GPU
        // ---------------------------------------------

        self.queue.submit(
            Some(encoder.finish())
        );

        // ---------------------------------------------
        // 6. Показуємо кадр
        // ---------------------------------------------

        self.queue.present(output);
    }
}