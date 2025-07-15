use std::{cell::RefCell, cmp, rc::Rc};

use winit::{event::WindowEvent, keyboard::Key};

use crate::{app::{app_owned_data::{InputListener, Updateable}, App}, mesh, shader::{BasicShader, Shader}};

const SPEED : f32 = 0.3;

enum MoveDir {
    Up,
    Left,
    Down,
    Right,
}

struct Position {
    pub x : f32,
    pub y : f32, 
}

pub struct Snek {
    mesh : mesh::UnitRect,
    shader : Rc<BasicShader>,
    position : Position,
    move_dir : MoveDir
}

impl Snek {
    pub fn new(app : &mut App, shader : Rc<BasicShader>)->Self {
        Snek {
            mesh: mesh::UnitRect::new(&app.gl, shader.as_ref()),
            position : Position { x: 0.5, y: 0.5 },
            move_dir : MoveDir::Left,
            shader
        }
    }

    fn process_move(&mut self, delta : &std::time::Duration) {

        let curr_x = self.position.x;
        let curr_y = self.position.y;
        
        let (new_x, new_y) = match self.move_dir {
            MoveDir::Down => {
                let new_y = curr_y - (SPEED * delta.as_secs_f32());
                (curr_x, f32::max(f32::min(1., new_y), 0.))
            }
            MoveDir::Up => {
                let new_y = curr_y + (SPEED * delta.as_secs_f32());
                (curr_x, f32::max(f32::min(1., new_y), 0.))
            }
            MoveDir::Left => {
                let new_x = curr_x - (SPEED * delta.as_secs_f32());
                (f32::max(f32::min(1., new_x), 0.), curr_y)
            }
            MoveDir::Right => {
                let new_x = curr_x + (SPEED * delta.as_secs_f32());
                (f32::max(f32::min(1., new_x), 0.), curr_y)
            }
            _ => (curr_x, curr_y),
        };
        self.position.x = new_x;
        self.position.y = new_y;
    } 
}

impl Updateable for Snek {
    fn on_setup(&mut self, gl: &glow::Context) {
        self.shader.use_shader(gl);
        self.shader.set_circle_radius(gl, 0.05);
    }
    fn on_tick(&mut self, gl : &glow::Context, delta : &std::time::Duration, since_0 : &std::time::Duration) {
        self.process_move(delta);
        
        self.shader.use_shader(gl);
        self.shader.set_circle_pos(gl, self.position.x, self.position.y);
        self.mesh.render(gl, delta, since_0);
    }
}

impl InputListener for Snek {
    fn on_input(&mut self, event: &winit::event::WindowEvent) {
        use winit::event::WindowEvent;
        use winit::keyboard::NamedKey;
        use winit::keyboard::Key;

        match event {
            WindowEvent::KeyboardInput { device_id : _, event, is_synthetic:_ } => {
                if !event.state.is_pressed() {
                    return;
                }
                match event.logical_key {
                    Key::Named(NamedKey::ArrowUp) => match self.move_dir {
                        MoveDir::Down => {},
                        _ => self.move_dir = MoveDir::Up
                    },
                    Key::Named(NamedKey::ArrowLeft) => match self.move_dir {
                        MoveDir::Right => {},
                        _ => self.move_dir = MoveDir::Left
                    },
                    Key::Named(NamedKey::ArrowRight) => match self.move_dir {
                        MoveDir::Left => {}
                        _ => self.move_dir = MoveDir::Right,
                    },
                    Key::Named(NamedKey::ArrowDown) => match self.move_dir {
                        MoveDir::Up => {}
                        _ => self.move_dir = MoveDir::Down,
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
}