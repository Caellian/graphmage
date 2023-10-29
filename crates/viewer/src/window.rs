use std::num::NonZeroU32;
use std::process::exit;

use glutin::config::ConfigTemplateBuilder;
use glutin_winit::DisplayBuilder;
use graphmage_render::context::{gl_config_selector, Context};
use raw_window_handle::HasRawWindowHandle;
use winit::event::{Event, WindowEvent};
use winit::event_loop::ControlFlow;
use winit::window::Window as WinitWindow;
use winit::{event_loop::EventLoop, window::WindowBuilder};

pub struct Window {
    event_loop: EventLoop<()>,
    window: WinitWindow,
    context: Context,
}

impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new().expect("unable to create event loop");

        let window_builder = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_resizable(true)
            .with_title("graphmage");

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs.reduce(gl_config_selector).unwrap()
            })
            .unwrap();

        let window = window.expect("unable to create window");

        let raw_window_handle = window.raw_window_handle();

        let (width, height) = {
            let (w, h): (u32, u32) = window.inner_size().into();
            (
                NonZeroU32::new(w).expect("expected non-zero window width"),
                NonZeroU32::new(h).expect("expected non-zero window height"),
            )
        };

        let mut context = Context::new_gl(gl_config, width, height, Some(raw_window_handle));
        context.set_display_scale(window.scale_factor() as f32);

        Window {
            event_loop,
            window,
            context,
        }
    }

    pub fn run<F: Fn(&mut Context)>(self, render: F) -> ! {
        let Window {
            event_loop,
            window,
            mut context,
        } = self;

        event_loop
            .run(move |event, target| {
                target.set_control_flow(ControlFlow::Poll);

                match event {
                    Event::LoopExiting => target.exit(),
                    Event::WindowEvent { ref event, .. } => match event {
                        #[cfg(not(target_arch = "wasm32"))]
                        WindowEvent::Resized(physical_size) => {
                            let (w, h) = Into::<(u32, u32)>::into(*physical_size);
                            let (w, h) = (w.max(1), h.max(1));

                            context
                                .set_size(NonZeroU32::new(w).unwrap(), NonZeroU32::new(h).unwrap());
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                            context.set_display_scale(*scale_factor as f32);
                        }
                        WindowEvent::RedrawRequested => {
                            render(&mut context);
                            #[cfg(not(target_arch = "wasm32"))]
                            context.swap_buffers().unwrap();
                        }
                        WindowEvent::CloseRequested => target.exit(),
                        _ => (),
                    },
                    Event::AboutToWait => window.request_redraw(),
                    _ => (),
                }
            })
            .expect("unable to run event loop");

        exit(0)
    }
}
