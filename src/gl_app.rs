use std::{cell::RefCell, collections::HashMap, mem, rc::Rc, time::{self, Duration}};

use glow::{HasContext, NativeBuffer, NativeVertexArray, COLOR_BUFFER_BIT, STATIC_DRAW};
use winit::{event::WindowEvent, keyboard::Key};

use crate::{mesh::{Mesh}, shader::{BasicShader, Shader}};

pub trait Renderable {
    fn render(&self, gl : &glow::Context, delta : &time::Duration, since_0 : &time::Duration);
}

pub struct GlApp {
    pub gl: glow::Context,
    pub renderable : Vec<Box<dyn Renderable>>,
    t_0 : time::SystemTime,
    t_last_render: time::SystemTime,

    input_listener : Rc<RefCell<HashMap<usize, Box<dyn Fn(&WindowEvent)>>>>,
    input_id : usize
}

impl GlApp {
    pub fn new(gl: glow::Context) -> Self {
        let basic_shader = Rc::new(BasicShader::new(&gl));
        let basic_mesh = Mesh::new(&gl, basic_shader);

        let renderer = Self {
            gl,
            t_last_render: time::SystemTime::now(),
            t_0: time::SystemTime::now(),
            renderable: vec![Box::new(basic_mesh)],
            input_listener: Rc::new(RefCell::new(HashMap::new())),
            input_id : 0,
        };
        
        renderer
    }
    pub fn add_input_listener(&mut self, listener : Box<dyn Fn(&WindowEvent)>) -> Box<impl FnOnce() + use<>> {
        let curr_id = self.input_id; 
        self.input_listener.borrow_mut().insert(curr_id, listener);
        
        self.input_id+=1;

        let listener_list = self.input_listener.clone();
        Box::new(move || {listener_list.borrow_mut().remove(&curr_id);})
    }

    pub fn render(&mut self) {
        let delta = self.calc_delta();

        unsafe {
            self.gl.clear_color(0., 0.5, 0.5, 1.);
            self.gl.clear(COLOR_BUFFER_BIT);
        }

        for r in &self.renderable {
            r.render(&self.gl, &delta, &self.elapsed());
        }
    }

    pub fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match &event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { device_id : _, event, is_synthetic : _ } => {
                if event.logical_key == Key::Named(winit::keyboard::NamedKey::Escape) {
                    event_loop.exit();
                }
            }
            _ => (),
        }
        for listener in self.input_listener.borrow_mut().values() {
            (listener)(&event);
        }
    }

    fn calc_delta(&mut self) -> time::Duration {
        let delta = time::SystemTime::now()
            .duration_since(self.t_last_render)
            .unwrap();
        self.t_last_render = time::SystemTime::now();
        delta
    }

    fn elapsed(&self) -> time::Duration {
        self.t_0.elapsed().unwrap()
    }
}