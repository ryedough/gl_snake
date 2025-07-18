use std::{num::NonZero, time::{self, Duration, SystemTime}};
use glutin::{
    context::NotCurrentContext, config::{Config, ConfigTemplateBuilder, GetGlConfig}, context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version}, display::GetGlDisplay, prelude::*, surface::{Surface, WindowSurface}
};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{
    application::ApplicationHandler, dpi::{LogicalSize, PhysicalSize}, error::EventLoopError, event::WindowEvent, event_loop::{ControlFlow, EventLoop}, keyboard::Key, raw_window_handle::HasWindowHandle, window::{Window, WindowAttributes}
};

use crate::{app::App, MAX_FPS, WINDOW_HEIGHT, WINDOW_WIDTH};

enum GlDisplayCreationState {
    Unbuilt(DisplayBuilder),
    AlreadyBuilt,
}

struct AppState {
    window: Window,
    gl_surface: Surface<WindowSurface>,
}

pub struct AppBootstraper {
    state: Option<AppState>,
    gl_context: Option<PossiblyCurrentContext>,
    gl_display: GlDisplayCreationState,
    template: ConfigTemplateBuilder,
    app : Option<App>,
    on_app_init : fn(&mut App),
    last_render : Option<SystemTime>,
}

const MAX_FPS_PROC:f32 = 1./MAX_FPS;

// responsible for creating & managing window & gl context
impl AppBootstraper {
    pub fn new(on_app_init : fn(&mut App)) -> Self {
        Self {
            state: None,
            gl_context: None,
            app : None,
            gl_display: GlDisplayCreationState::Unbuilt(DisplayBuilder::new().with_window_attributes(Some(window_attributes()))),
            template: ConfigTemplateBuilder::default().prefer_hardware_accelerated(Some(true)),
            on_app_init,
            last_render: None,
        }
    }

    pub fn exec(mut self) -> Result<(), EventLoopError> {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(&mut self)
    } 
}

impl ApplicationHandler for AppBootstraper {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let (window, gl_config) = match &self.gl_display {
            GlDisplayCreationState::Unbuilt(display_builder) => {
                let (window, config) = display_builder
                    .clone()
                    .build(event_loop, self.template.clone(), gl_config_picker)
                    .unwrap();
                let window = window.unwrap();

                self.gl_context = Some(create_gl_context(&window, &config).treat_as_possibly_current());
                
                self.gl_display = GlDisplayCreationState::AlreadyBuilt;

                (window, config)
            }
            GlDisplayCreationState::AlreadyBuilt => {
                let gl_config = self.gl_context.as_ref().unwrap().config();
                match glutin_winit::finalize_window(event_loop, window_attributes(), &gl_config) {
                    Ok(window) => (window, gl_config),
                    Err(_) => {
                        return event_loop.exit();
                    }
                }
            }
        };

        let attrs = window.build_surface_attributes(Default::default()).unwrap();
        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = self.gl_context.as_ref().unwrap();
        gl_context.make_current(&gl_surface).unwrap();

        //create app
        self.app.get_or_insert_with(|| {
            let gl = unsafe { glow::Context::from_loader_function_cstr(|s|self.gl_context.as_ref().unwrap().display().get_proc_address(s)) };
            let mut app = App::new(gl, self.on_app_init);
            app.after_on_app_init();
            app
        });

        assert!(
            self.state
                .replace(AppState { gl_surface, window })
                .is_none()
        );
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match &event {
            WindowEvent::Resized(size) if size.width != 0 && size.height != 0 => {
                if let Some(AppState {
                    window: _,
                    gl_surface,
                }) = self.state.as_ref()
                {
                    let gl_context = self.gl_context.as_ref().unwrap();
                    gl_surface.resize(
                        gl_context,
                        NonZero::new(size.width).unwrap(),
                        NonZero::new(size.height).unwrap(),
                    );
                }
            },
            winit::event::WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            winit::event::WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if event.logical_key
                    == winit::keyboard::Key::Named(winit::keyboard::NamedKey::Escape)
                {
                    event_loop.exit();
                }
            }
            _=> {},
        }
        self.app.as_mut().map(|app|app.window_event(event_loop, window_id, event));
    }
    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        //clear window
        self.state = None;
        self.app.as_mut().map(|app| app.on_exit());

        #[cfg(egl_backend)]
        {
            //fix for nvidia wayland
            let _gl_display = self.gl_context.take().unwrap().display();
            if let glutin::display::Display::Egl(display) = _gl_display {
                unsafe {
                    display.terminate();
                }
            }
        }
    }
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let last_render = self.last_render.get_or_insert(time::SystemTime::now());
        if let Some(AppState { gl_surface, window }) = self.state.as_ref() 
            && time::SystemTime::now().duration_since(*last_render).unwrap().as_secs_f32() > MAX_FPS_PROC{
            let _ = self.last_render.insert(SystemTime::now());
            let gl_context = self.gl_context.as_ref().unwrap();
            self.app.as_mut().map(|r|r.render());
            window.request_redraw();

            gl_surface.swap_buffers(gl_context).unwrap();
        }
    }
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

fn window_attributes() -> WindowAttributes {
    Window::default_attributes()
        .with_transparent(true)
        .with_title("GL_Snek")
        .with_resizable(false)
        .with_inner_size(PhysicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display.create_context(gl_config, &context_attributes).unwrap_or_else(|_| {
            gl_display.create_context(gl_config, &fallback_context_attributes).unwrap_or_else(
                |_| {
                    gl_display
                        .create_context(gl_config, &legacy_context_attributes)
                        .expect("failed to create context")
                },
            )
        })
    }
}
