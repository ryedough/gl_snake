use core::time;

use crate::app::RegisteredCollider;

pub enum AppOwnedData {
    Updateable(Box<dyn Updateable>),
    InputListener(Box<dyn InputListener>),
    UpdateableInputListener(Box<dyn UpdateableInputListener>),
}

impl AppOwnedData {
    pub fn as_updateable(&mut self) -> Option<&mut dyn Updateable> {
        match self {
            Self::Updateable(x) => Some(x.as_mut()),
            Self::UpdateableInputListener(x) => Some(x.as_mut()),
            _ => None,
        }
    }
    pub fn as_input_listener(&mut self) -> Option<&mut dyn InputListener> {
        match self {
            Self::InputListener(x) => Some(x.as_mut()),
            Self::UpdateableInputListener(x) => Some(x.as_mut()),
            _ => None,
        }
    }

    pub fn from_updateable(data : impl Updateable + 'static) -> Self {
        Self::Updateable(Box::new(data))
    }
    pub fn from_input_listener(data : impl InputListener + 'static) -> Self {
        Self::InputListener(Box::new(data))
    }
    pub fn from_updt_input(data : impl UpdateableInputListener + 'static) -> Self {
        Self::UpdateableInputListener(Box::new(data))
    }
}

pub trait UpdateableInputListener: Updateable + InputListener {}
impl<T: InputListener + Updateable> UpdateableInputListener for T {}

pub trait InputListener {
    fn on_input(&mut self, event: &winit::event::WindowEvent);
}

pub trait Updateable {
    /// can also render inside this function
    fn on_tick(&mut self, gl: &glow::Context, delta: &time::Duration, since_0: &time::Duration);
    fn on_setup(&mut self, gl: &glow::Context);
}

pub trait Collider {
    fn on_collide(&mut self, collider : &impl Collider);
    fn identifier(&self) -> RegisteredCollider;
}