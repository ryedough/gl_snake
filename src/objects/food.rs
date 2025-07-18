use rand::Rng;

use crate::{app::{app_owned_data::{Collider, Setupable, Updateable}, board::Position, collider::{ColliderType, AABB}, ColliderLayer}, meshes::UnitRect, shaders::{FoodShader, Shader}};

pub struct Food {
    shader : FoodShader,
    mesh : UnitRect,
    radius : f32,
    position : Position,
    collided_with_player : bool,
}

impl Food {
    pub fn new(gl : &glow::Context)->Food{
        let shader = FoodShader::new(gl);
        let mesh = UnitRect::new(gl, &shader);

        Food { 
            shader,
            mesh,
            radius: 0.,
            position: Position::default(),
            collided_with_player: true,
        }
    }
    fn get_new_pos(&mut self, board: &crate::app::board::Board) {
        let mut rng = rand::rng();
        let x = rng.random_range(0..board.midpoints.len());
        let y = rng.random_range(0..board.midpoints[0].1.len());
        let y = board.midpoints[x].1[y];
        let x = board.midpoints[x].0;
        self.position = Position{
            x, y 
        };
    }
}

impl Setupable for Food {
    fn on_setup(&mut self, gl: &glow::Context, registered_idx:usize, board: &crate::app::board::Board) {
        self.radius = board.grid_size/2.;
        self.shader.use_shader(gl);
        
        self.get_new_pos(board);
        
        self.shader.set_radius(gl, self.radius);
        self.shader.set_position(gl, self.position.x, self.position.y);
    }
}

impl Updateable for Food {
    fn on_tick(&mut self, gl: &glow::Context, time : &crate::app::app_owned_data::Time, board: &crate::app::board::Board, _ : &mut dyn FnMut()) {
        if self.collided_with_player {
            self.get_new_pos(board);
            self.collided_with_player = false;
        }
        self.shader.use_shader(gl);
        self.shader.set_time(gl, (time.elapsed.as_secs_f64() * 10.).sin() as f32);
        self.shader.set_position(gl, self.position.x, self.position.y);
        self.mesh.render(gl);
    }
}

impl Collider for Food {
    fn check_collision(&mut self, other: &dyn Collider) {
        if other.layer() != ColliderLayer::Player {
            return;
        }
        let self_cldr = &self.collider()[0];
        let ColliderType::AABB(self_cldr) = self_cldr;

        let is_intersect = other.collider().iter().any(|other| match other {
            ColliderType::AABB(other) => self_cldr.intersects(other),
        });
        if is_intersect {
            self.collided_with_player = true;
        };
    }
    fn layer(&self) -> crate::app::ColliderLayer {
        crate::app::ColliderLayer::Food
    }
    fn collider(&self) -> Vec<crate::app::collider::ColliderType> {
        let c = ColliderType::AABB(AABB::new(
            Position { x: self.position.x - self.radius, y: self.position.y - self.radius }, 
            Position { x: self.position.x + self.radius, y: self.position.y + self.radius }
        ));
        vec![c]
    }
}