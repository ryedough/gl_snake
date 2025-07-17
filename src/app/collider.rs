use crate::app::board::Position;

// add new collider here for different type
#[derive(Clone, Copy, PartialEq)]
pub enum ColliderLayer {
    Player,
    Food
}

#[derive(Debug)]
pub enum ColliderType {
    AABB(AABB)
}

#[derive(Debug)]
pub struct AABB {
    min: Position,
    max: Position,
}

impl AABB {
    pub fn new(min: Position, max: Position) -> Self {
        AABB { min, max }
    }

    pub fn intersects(&self, other: &AABB) -> bool {
        !(self.max.x < other.min.x || self.min.x > other.max.x ||
          self.max.y < other.min.y || self.min.y > other.max.y)
    }
}