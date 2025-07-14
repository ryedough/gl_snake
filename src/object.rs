use std::{cell::RefCell, rc::Rc};

use winit::{event::WindowEvent, keyboard::Key};

use crate::{gl_app::{GlApp, InputListener, Updateable}, mesh::Mesh, shader::BasicShader};

pub struct Square {
    mesh : Mesh<BasicShader>,
    rotation : f32,
}

impl Square {
    pub fn new(app : &mut GlApp, shader : Rc<BasicShader>)->Self {
        Square {
            mesh: Mesh::new(&app.gl, shader),
            rotation: 0.,
        }
    }
}

impl Updateable for Square {
    fn on_tick(&mut self, gl : &glow::Context, delta : &std::time::Duration, since_0 : &std::time::Duration) {
        
    }
}

impl InputListener for Square {
    fn on_input(&mut self, event: &winit::event::WindowEvent) {
        use winit::event::WindowEvent;
        use winit::keyboard::NamedKey;

        match event {
            WindowEvent::KeyboardInput { device_id : _, event, is_synthetic:_ } => {
                if event.logical_key == NamedKey::ArrowUp {
                    println!("poxa!");
                };
            },
            _ => {}
        }
    }
}