use std::{cell::RefCell, rc::Rc};

use winit::{event::WindowEvent, keyboard::Key};

use crate::{gl_app::{GlApp, Renderable}, mesh::Mesh, shader::BasicShader};

pub struct Square {
    mesh : Mesh<BasicShader>,
    cancel_input_listener : Option<Box<dyn FnOnce()>>,
    rotation : Rc<RefCell<f32>>,
}

impl Square {
    pub fn new(app : &mut GlApp, shader : Rc<BasicShader>)->Self {
        let rotation = Rc::new(RefCell::new(0.0));
        let rotation_clone = Rc::clone(&rotation);
        let cancel_input = app.add_input_listener(Box::new(move |e| {
            match e {
                WindowEvent::KeyboardInput { device_id:_, event, is_synthetic: _ } => {
                    if event.logical_key == Key::Named(winit::keyboard::NamedKey::ArrowUp) {
                        println!("Bossanova!");
                    };
                    *rotation_clone.borrow_mut() += 0.1;
                },
                _=>{}
            };
        }));
        Square {
            mesh: Mesh::new(&app.gl, shader),
            cancel_input_listener: Some(cancel_input),
            rotation: rotation.clone(),
        }
    }
}

impl Renderable for Square {
    fn render(&self, gl : &glow::Context, delta : &std::time::Duration, since_0 : &std::time::Duration) {
        
    }
}

impl Drop for Square{
    fn drop(&mut self) {
        if let Some(cancel_input) = self.cancel_input_listener.take() {
            cancel_input();
        }
    }
}