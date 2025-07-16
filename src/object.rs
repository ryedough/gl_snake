use crate::{
    app::{
        App,
        app_owned_data::{InputListener, Updateable},
    },
    mesh,
    shader::{BasicShader, Shader},
};
use std::collections::VecDeque;
use std::rc::Rc;

const SPEED: f32 = 0.3;

#[derive(Clone, Debug)]
enum MoveDir {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Clone, Debug)]
struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
struct MoveDirChange {
    direction: MoveDir,
    at: Position,
    dst_from_head: f32,
}

pub struct Snek {
    mesh: mesh::UnitRect,
    shader: Rc<BasicShader>,
    position: Position,
    dir: MoveDir,
    dir_candidate: Option<MoveDir>,
    dir_keypoints: VecDeque<MoveDirChange>,
    length: f32,
}

impl Snek {
    pub fn new(app: &mut App, shader: Rc<BasicShader>) -> Self {
        Snek {
            mesh: mesh::UnitRect::new(&app.gl, shader.as_ref()),
            position: Position { x: 0.5, y: 0.5 },
            dir: MoveDir::Left,
            dir_keypoints: VecDeque::new(),
            dir_candidate: None,
            length: 0.1,
            shader,
        }
    }

    fn process_move(&mut self, move_dist: f32) {
        let curr_x = self.position.x;
        let curr_y = self.position.y;

        let (new_x, new_y) = match self.dir_candidate.as_ref().unwrap_or(&self.dir) {
            MoveDir::Down => {
                let new_y = curr_y - move_dist;
                (curr_x, f32::max(f32::min(1., new_y), 0.))
            }
            MoveDir::Up => {
                let new_y = curr_y + move_dist;
                (curr_x, f32::max(f32::min(1., new_y), 0.))
            }
            MoveDir::Left => {
                let new_x = curr_x - move_dist;
                (f32::max(f32::min(1., new_x), 0.), curr_y)
            }
            MoveDir::Right => {
                let new_x = curr_x + move_dist;
                (f32::max(f32::min(1., new_x), 0.), curr_y)
            }
            _ => (curr_x, curr_y),
        };
        self.position.x = new_x;
        self.position.y = new_y;
    }
    fn process_dir_keypoints(&mut self, move_dist: f32) {
        for changes in &mut self.dir_keypoints {
            changes.dst_from_head += move_dist
        }

        let snek_length = self.length;
        while let Some(front) = self.dir_keypoints.front()
            && front.dst_from_head > snek_length
        {
            self.dir_keypoints.pop_front();
        }
    }
}

impl Updateable for Snek {
    fn on_setup(&mut self, gl: &glow::Context) {
        self.shader.use_shader(gl);
        self.shader.set_circle_radius(gl, 0.05);
    }
    fn on_tick(
        &mut self,
        gl: &glow::Context,
        delta: &std::time::Duration,
        since_0: &std::time::Duration,
    ) {
        self.shader.use_shader(gl);

        let move_dist = SPEED * delta.as_secs_f32();
        self.process_move(move_dist);
        self.process_dir_keypoints(move_dist);
        self.dir_candidate.take().map(|dir| {
            // process new direction fired from keyboard
            self.dir = dir.clone();
            self.dir_keypoints.push_back(MoveDirChange {
                direction: dir,
                at: self.position.clone(),
                dst_from_head: 0.0,
            });
        });
        self.shader
            .set_circle_pos(gl, self.position.x, self.position.y);
        self.mesh.render(gl, delta, since_0);
    }
}

impl InputListener for Snek {
    fn on_input(&mut self, event: &winit::event::WindowEvent) {
        use winit::event::WindowEvent;
        use winit::keyboard::{KeyCode, PhysicalKey};

        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                if !event.state.is_pressed() {
                    return;
                }
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::ArrowUp) | PhysicalKey::Code(KeyCode::KeyW) => {
                        match self.dir {
                            MoveDir::Down => {}
                            _ => {
                                self.dir_candidate.get_or_insert(MoveDir::Up);
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) | PhysicalKey::Code(KeyCode::KeyA) => {
                        match self.dir {
                            MoveDir::Right => {}
                            _ => {
                                self.dir_candidate.get_or_insert(MoveDir::Left);
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) | PhysicalKey::Code(KeyCode::KeyD) => {
                        match self.dir {
                            MoveDir::Left => {}
                            _ => {
                                self.dir_candidate.get_or_insert(MoveDir::Right);
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) | PhysicalKey::Code(KeyCode::KeyS) => {
                        match self.dir {
                            MoveDir::Up => {}
                            _ => {
                                self.dir_candidate.get_or_insert(MoveDir::Down);
                            }
                        }
                    }
                    _ => {}
                };
            }
            _ => {}
        }
    }
}
