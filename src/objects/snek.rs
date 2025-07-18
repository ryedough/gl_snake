use crate::{
    app::{
        App, ColliderLayer,
        app_owned_data::{Collider, InputListener, Setupable, Updateable},
        board::{Board, Position},
        collider::{AABB, ColliderType},
    },
    meshes,
    shaders::{Shader, SnekShader},
};
use std::{cmp, collections::VecDeque, time::Duration};
use std::rc::Rc;

#[repr(u8)]
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum MoveDir {
    Up,
    Right,
    Down,
    Left,
}

impl MoveDir {
    fn invert(&self) -> Self{
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DirKeypoint {
    pub from: MoveDir,
    pub at: Position,
    pub dst_head: f32,
}

const GRID_TRESHOLD: f32 = 4.;
const INIT_LENGTH: f32 = 100.;
const LENGTH_PER_FOOD: f32 = 10.;
const INIT_SPEED: f32 = 120.;
const MAX_SPEED:f32 = 160.;
const SPEED_PER_FOOD: f32 = 3.;
const MAX_DURATION_ON_EDGE:f32 = 0.1;

pub struct Snek {
    mesh: meshes::UnitRect,
    shader: Rc<SnekShader>,
    position: Position,
    dir: MoveDir,
    dir_candidate: Option<MoveDir>,
    dir_keypoints: VecDeque<DirKeypoint>,
    length: f32,
    radius: f32,
    speed: f32,
    in_edge : Duration,
    game_over : bool,
}

impl Snek {
    pub fn new(app: &mut App, shader: Rc<SnekShader>) -> Self {
        Snek {
            mesh: meshes::UnitRect::new(&app.gl, shader.as_ref()),
            position: Position::default(),
            dir: MoveDir::Left,
            dir_keypoints: VecDeque::new(),
            dir_candidate: None,
            length: INIT_LENGTH,
            speed: INIT_SPEED,
            shader,
            radius: 0.,
            in_edge : Duration::from_secs(0),
            game_over : false,
        }
    }

    fn process_move(&mut self, board: &Board, move_dist: f32) {
        let curr_x = self.position.x;
        let curr_y = self.position.y;

        let (new_x, new_y) = match self.dir {
            MoveDir::Down => {
                let new_y = curr_y - move_dist;
                (
                    curr_x,
                    f32::max(
                        f32::min(board.height - self.radius, new_y),
                        0. + self.radius,
                    ),
                )
            }
            MoveDir::Up => {
                let new_y = curr_y + move_dist;
                (
                    curr_x,
                    f32::max(
                        f32::min(board.height - self.radius, new_y),
                        0. + self.radius,
                    ),
                )
            }
            MoveDir::Left => {
                let new_x = curr_x - move_dist;
                (
                    f32::max(f32::min(board.width - self.radius, new_x), 0. + self.radius),
                    curr_y,
                )
            }
            MoveDir::Right => {
                let new_x = curr_x + move_dist;
                (
                    f32::max(f32::min(board.width - self.radius, new_x), 0. + self.radius),
                    curr_y,
                )
            }
        };
        self.position.x = new_x;
        self.position.y = new_y;
    }
    fn process_dir_keypoints(&mut self, move_dist: f32) {
        self.dir_keypoints.make_contiguous();
        for changes in &mut self.dir_keypoints {
            changes.dst_head += move_dist
        }

        let snek_length = self.length;
        while let Some(front) = self.dir_keypoints.front()
            && front.dst_head > snek_length
        {
            self.dir_keypoints.pop_front();
        }
    }
    fn get_keypoints(&self) -> Vec<DirKeypoint> {
        let curr_point = DirKeypoint {
            at: self.position.clone(),
            dst_head: 0.,
            from: self.dir.clone().invert(),
        };

        let kp_slice = self.dir_keypoints.as_slices();
        [kp_slice.0, kp_slice.1, &[curr_point]].concat()
    }
    fn on_edge(&mut self, delta : &Duration) {
        if self.in_edge > Duration::from_secs_f32(MAX_DURATION_ON_EDGE){
            self.game_over = true;
        }else{
            self.in_edge += *delta;
        }
    }
}

impl Setupable for Snek {
    fn on_setup(&mut self, gl: &glow::Context, registered_idx: usize, board: &Board) {
        self.position = Position {
            x: board.width / 2.,
            y: board.height / 2.,
        };
        self.radius = board.grid_size / 2.;

        self.shader.use_shader(gl);
        self.shader.set_circle_radius(gl, self.radius);
        self.shader.set_length(gl, self.length);
    }
}

impl Updateable for Snek {
    fn on_tick(
        &mut self,
        gl: &glow::Context,
        time: &crate::app::app_owned_data::Time,
        board: &Board,
        game_over : &mut dyn FnMut(),
    ) {
        if self.game_over {
            return game_over();
        }
        self.shader.use_shader(gl);

        let move_dist = self.speed * time.delta.as_secs_f32();
        self.process_move(board, move_dist);

        match self.dir {
            MoveDir::Right if self.position.x == board.width - self.radius => {self.on_edge(time.delta)},
            MoveDir::Left if self.position.x == self.radius => {self.on_edge(time.delta)},
            MoveDir::Up if self.position.y == board.height - self.radius => {self.on_edge(time.delta)},
            MoveDir::Down if self.position.y == self.radius => {self.on_edge(time.delta)},
            _ => {self.process_dir_keypoints(move_dist); self.in_edge = Duration::from_secs(0)}
        }

        let current_midpoint = board.current_midpts(self.position.clone()).unwrap();
        let last_move_midpoint = if let Some(last_move) = self.dir_keypoints.back() {
            board.current_midpts(last_move.at.clone())
        } else {
            None
        };

        if self.dir_candidate.is_some()
            && last_move_midpoint.is_none_or(|lm| lm != current_midpoint)
        {
            let mut adjusted_position: Option<Position> = None;
            let treshold = (GRID_TRESHOLD + ((self.speed / INIT_SPEED)-1.) *3.).clamp(0., board.grid_size * 0.9);
            //TODO: correct keypoint after correction
            let correction;
            match self.dir {
                MoveDir::Up | MoveDir::Down => {
                    correction = current_midpoint.y - self.position.y;
                    if correction.abs() < treshold {
                        adjusted_position = Some(Position {
                            x: self.position.x,
                            y: current_midpoint.y,
                        });
                    }
                }
                MoveDir::Left | MoveDir::Right => {
                    correction = current_midpoint.x - self.position.x;
                    if correction.abs() < treshold {
                        adjusted_position = Some(Position {
                            x: current_midpoint.x,
                            y: self.position.y,
                        });
                    }
                }
            }
            if let Some(pos) = adjusted_position {
                self.dir_candidate.take().map(|dir| {
                    // process new direction fired from keyboard
                    self.dir_keypoints.push_back(DirKeypoint {
                        from: self.dir.clone().invert(),
                        at: pos.clone(),
                        dst_head: 0.0,
                    });
                    self.dir = dir;
                    self.position = pos;
                });
            }
        }

        self.shader.set_length(gl, self.length);
        self.shader.set_keypoints(gl, &self.get_keypoints());
        self.mesh.render(gl);
    }
}

impl Collider for Snek {
    fn layer(&self) -> ColliderLayer {
        ColliderLayer::Player
    }
    fn check_collision(&mut self, other: &dyn Collider) {
        if other.layer() != ColliderLayer::Food {
            return;
        }

        let head_cldr = AABB::new(
            Position {
                x : self.position.x - self.radius * 0.9,
                y : self.position.y - self.radius * 0.9,
            },
            Position { 
                x : self.position.x + self.radius * 0.9,
                y : self.position.y + self.radius * 0.9,
            },
        );
        let self_cldr= self.collider();
        let self_cldr = self_cldr.split_at_checked(2);
        let mut head_collide_self = false;
        match self_cldr {
            Some((_,self_cldr)) => {
                head_collide_self = self_cldr.iter().any(|self_cldr|match self_cldr {
                    ColliderType::AABB(self_cldr) => {
                        self_cldr.intersects(&head_cldr)
                    },
                });
            },
            None => {},
        }
        if head_collide_self {
            self.game_over = true;
            return;
        }

        let head_collide = other.collider().iter().any(
            |other_cldr| match other_cldr {
                ColliderType::AABB(other_aabb) => head_cldr.intersects(other_aabb)});
        if head_collide {
            self.length += LENGTH_PER_FOOD;
            self.speed += SPEED_PER_FOOD.clamp(0., MAX_SPEED);
        };
    }
    fn collider(&self) -> Vec<ColliderType> {
        let mut remaining_len = self.length;
        let keypoints = self.get_keypoints();
        let mut res = Vec::with_capacity(keypoints.len());
        for i in (1..keypoints.len()).rev(){
            let (n_1, n) = keypoints.split_at(i);
            let (n_1, n) = (&n_1[i-1], &n[0]);
            let len = n_1.dst_head - n.dst_head; 
            remaining_len -= len;

            let (small_x, big_x) = match n.at.x.total_cmp(&n_1.at.x) {
                cmp::Ordering::Less => (n.at.x, n_1.at.x),
                _ => (n_1.at.x, n.at.x),
            };
            let (small_y, big_y) = match n.at.y.total_cmp(&n_1.at.y) {
                cmp::Ordering::Less => (n.at.y, n_1.at.y),
                _ => (n_1.at.y, n.at.y),
            };
            let aabb = AABB::new(
                Position {
                    x: small_x - self.radius * 0.9,
                    y: small_y - self.radius * 0.9,
                },
                Position {
                    x: big_x + self.radius * 0.9,
                    y: big_y + self.radius * 0.9,
                },
            );
            res.push(ColliderType::AABB(aabb));
        };
        let aabb = match keypoints[0].from {
            MoveDir::Down => {
                let small_x = keypoints[0].at.x - self.radius * 0.9;
                let big_x = keypoints[0].at.x + self.radius * 0.9;
                AABB::new(
                    Position {
                        x: small_x,
                        y: keypoints[0].at.y - remaining_len,
                    },
                    Position {
                        x: big_x,
                        y: keypoints[0].at.y + self.radius * 0.9,
                    },
                )
            },
            MoveDir::Up => {
                let small_x = keypoints[0].at.x - self.radius * 0.9;
                let big_x = keypoints[0].at.x + self.radius * 0.9;
                AABB::new(
                    Position {
                        x: small_x,
                        y: keypoints[0].at.y - self.radius * 0.9,
                    },
                    Position {
                        x: big_x,
                        y: keypoints[0].at.y + remaining_len,
                    },
                )
            },
            MoveDir::Left => {
                let small_y = keypoints[0].at.y - self.radius * 0.9;
                let big_y = keypoints[0].at.y + self.radius * 0.9;
                AABB::new(
                    Position {
                        x: keypoints[0].at.x - remaining_len,
                        y: small_y,
                    },
                    Position {
                        x: keypoints[0].at.x + self.radius * 0.9,
                        y: big_y,
                    },
                )
            },
            MoveDir::Right => {
                let small_y = keypoints[0].at.y - self.radius * 0.9;
                let big_y = keypoints[0].at.y + self.radius * 0.9;
                AABB::new(
                    Position {
                        x: keypoints[0].at.x - self.radius * 0.9,
                        y: small_y,
                    },
                    Position {
                        x: keypoints[0].at.x + remaining_len,
                        y: big_y,
                    },
                )
            }
        };
        res.push(ColliderType::AABB(aabb));
        res
    }
}

impl InputListener for Snek {
    fn on_input(&mut self, event: &winit::event::WindowEvent, board : &Board) {
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
                            MoveDir::Down | MoveDir::Up => {}
                            _ if self.position.y > board.height - board.grid_size => {}
                            _ => {
                                self.dir_candidate = Some(MoveDir::Up);
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::ArrowLeft) | PhysicalKey::Code(KeyCode::KeyA) => {
                        match self.dir {
                            MoveDir::Right | MoveDir::Left => {}
                            _ if self.position.x < board.grid_size => {}
                            _ => {
                                self.dir_candidate = Some(MoveDir::Left);
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::ArrowRight) | PhysicalKey::Code(KeyCode::KeyD) => {
                        match self.dir {
                            MoveDir::Left | MoveDir::Right => {}
                            _ if self.position.x > board.width - board.grid_size => {}
                            _ => {
                                self.dir_candidate = Some(MoveDir::Right);
                            }
                        }
                    }
                    PhysicalKey::Code(KeyCode::ArrowDown) | PhysicalKey::Code(KeyCode::KeyS) => {
                        match self.dir {
                            MoveDir::Up | MoveDir::Down => {}
                            _ if self.position.y < board.grid_size => {}
                            _ => {
                                self.dir_candidate = Some(MoveDir::Down);
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
