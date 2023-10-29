use std::{
    cell::{RefCell, RefMut},
    num::NonZeroU32,
};

use glutin::{
    config::Config as GLConfig,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::GetGlDisplay,
    prelude::*,
    surface::{PbufferSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use raw_window_handle::RawWindowHandle;

pub type Canvas = femtovg::Canvas<femtovg::renderer::OpenGl>;

pub enum GLRenderSurface {
    Window(Surface<WindowSurface>),
    Buffer(Surface<PbufferSurface>),
}

pub struct Context {
    width: NonZeroU32,
    height: NonZeroU32,
    display_scale: f32,

    gl_config: GLConfig,
    surface: GLRenderSurface,
    gl_context: PossiblyCurrentContext,

    canvas: RefCell<Canvas>,
}

pub fn gl_config_selector(accum: GLConfig, config: GLConfig) -> GLConfig {
    // Find the config with the maximum number of samples, so our triangle will
    // be smooth.

    let transparency_check = config.supports_transparency().unwrap_or(false)
        & !accum.supports_transparency().unwrap_or(false);

    if transparency_check || config.num_samples() < accum.num_samples() {
        config
    } else {
        accum
    }
}

impl Context {
    pub fn new_gl(
        gl_config: GLConfig,
        width: NonZeroU32,
        height: NonZeroU32,
        window_handle: Option<RawWindowHandle>,
    ) -> Self {
        let gl_display = gl_config.display();

        let context_attributes = ContextAttributesBuilder::new().build(window_handle);
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(window_handle);

        let mut not_current_gl_context = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_display
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };

        let surface = match window_handle {
            Some(raw_window_handle) => {
                let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
                    raw_window_handle,
                    width,
                    height,
                );

                GLRenderSurface::Window(unsafe {
                    gl_display
                        .create_window_surface(&gl_config, &attrs)
                        .expect("can't create window surface")
                })
            }
            None => {
                let attrs = SurfaceAttributesBuilder::<PbufferSurface>::new().build(width, height);

                GLRenderSurface::Buffer(unsafe {
                    gl_display
                        .create_pbuffer_surface(&gl_config, &attrs)
                        .expect("can't create buffer surface")
                })
            }
        };

        let gl_context = match &surface {
            GLRenderSurface::Window(surface) => {
                not_current_gl_context.make_current(surface).unwrap()
            }
            GLRenderSurface::Buffer(surface) => {
                not_current_gl_context.make_current(surface).unwrap()
            }
        };

        let renderer = unsafe {
            femtovg::renderer::OpenGl::new_from_function_cstr(|s| {
                gl_display.get_proc_address(s) as *const _
            })
        }
        .expect("Cannot create renderer");

        let canvas = {
            let mut c = Canvas::new(renderer).expect("cannot create canvas");
            c.set_size(width.get(), height.get(), 1.0);

            let _ = c.add_font("resources/Roboto-Regular.ttf");

            RefCell::new(c)
        };

        Context {
            width: width,
            height: height,
            display_scale: 1.0,

            gl_config,
            surface,
            gl_context,

            canvas,
        }
    }

    pub fn set_size(&mut self, width: NonZeroU32, height: NonZeroU32) {
        self.width = width;
        self.height = height;

        match &mut self.surface {
            GLRenderSurface::Window(surface) => surface.resize(&self.gl_context, width, height),
            GLRenderSurface::Buffer(surface) => {
                let gl_display = self.gl_config.display();

                let attrs = SurfaceAttributesBuilder::<PbufferSurface>::new().build(width, height);
                *surface = unsafe {
                    gl_display
                        .create_pbuffer_surface(&self.gl_config, &attrs)
                        .expect("can't create buffer surface")
                };
            }
        }
        self.canvas
            .borrow_mut()
            .set_size(width.get(), height.get(), self.display_scale);
    }

    pub fn size(&self) -> (u32, u32) {
        (self.width.get(), self.height.get())
    }

    pub fn set_display_scale(&mut self, scale: f32) {
        self.display_scale = scale;
        self.canvas
            .borrow_mut()
            .set_size(self.width.get(), self.height.get(), scale);
    }

    pub fn swap_buffers(&self) -> Result<(), glutin::error::Error> {
        match &self.surface {
            GLRenderSurface::Window(surface) => surface.swap_buffers(&self.gl_context),
            GLRenderSurface::Buffer(surface) => surface.swap_buffers(&self.gl_context),
        }
    }

    pub fn surface(&self) -> &GLRenderSurface {
        &self.surface
    }

    pub fn canvas(&self) -> RefMut<'_, Canvas> {
        self.canvas.borrow_mut()
    }
}

/*
fn draw_frame(canvas: &mut Canvas, kind: &CtxKind) {
    let window_size = kind.size();

    canvas.clear_rect(
        0,
        0,
        window_size.0,
        window_size.1,
        Color::rgbf(1.0, 1.0, 1.0),
    );

    let line = Line {
        end: Vec2::new(512.0, 200.0),
        ..Default::default()
    };

    line.draw(&mut canvas);

    Node {
        pos: Vec2::new(50.0, 50.0),
    }
    .draw(&mut canvas);

    canvas.flush();
}
 */
