use core::time;
use std::any::Any;

use crate::app::RegisteredCollider;

pub struct AppOwnedData(Box<dyn Any>);

impl AppOwnedData {
    /// # Warning
    /// data must be trait object of type : Updateable, Collider, InputListener, or permutaition of those <br/>
    /// if object implement more than one trait use the permutation version, so it will be registered as both trait <br/>
    /// eg: `Updateable + Collider` use `CldrUpdt` 
    pub fn from<T :?Sized + 'static>(data : Box<T>) -> Self {
        AppOwnedData(Box::new(data))
    }
    pub fn is_updateable(&self) -> bool {
        self.is::<dyn Updateable>()
            || self.is::<dyn UpdtInpLstr>()
            || self.is::<dyn CldrUpdt>()
            || self.is::<dyn CldrUpdtInpLstr>()
    }
    pub fn is_collider(&self) -> bool {
        self.is::<dyn Collider>()
            || self.is::<dyn CldrInpLstr>()
            || self.is::<dyn CldrUpdt>()
            || self.is::<dyn CldrUpdtInpLstr>()
    }
    pub fn is_input_listener(&self) -> bool {
        self.is::<dyn InputListener>()
            || self.is::<dyn CldrInpLstr>()
            || self.is::<dyn UpdtInpLstr>()
            || self.is::<dyn CldrUpdtInpLstr>()
    }
    pub fn as_updateable(&mut self) -> Option<&mut dyn Updateable>{
        //TODO: make these macro variadic
        macro_rules! try_downcast {
            ($ty:ty) => {
                if self.is::<$ty>(){
                    let data = self.as_mut::<$ty>()?; 
                    return Some(data.as_mut());
                }
            }
        }
        
        try_downcast!(dyn Updateable);
        try_downcast!(dyn UpdtInpLstr);
        try_downcast!(dyn CldrUpdt);
        try_downcast!(dyn CldrUpdtInpLstr);
        return None
    }
    pub fn as_collider(&mut self) -> Option<&mut dyn Collider>{
        macro_rules! try_downcast {
            ($ty:ty) => {
                if self.is::<$ty>(){
                    let data = self.as_mut::<$ty>()?; 
                    return Some(data.as_mut());
                }
            }
        }
        
        try_downcast!(dyn Collider);
        try_downcast!(dyn CldrInpLstr);
        try_downcast!(dyn CldrUpdt);
        try_downcast!(dyn CldrUpdtInpLstr);
        return None
    }
    pub fn as_input_listener(&mut self) -> Option<&mut dyn InputListener>{
        macro_rules! try_downcast {
            ($ty:ty) => {
                if self.is::<$ty>(){
                    let data = self.as_mut::<$ty>()?; 
                    return Some(data.as_mut());
                }
            }
        }
        
        try_downcast!(dyn InputListener);
        try_downcast!(dyn CldrInpLstr);
        try_downcast!(dyn UpdtInpLstr);
        try_downcast!(dyn CldrUpdtInpLstr);
        return None
    }
    fn is<T: ?Sized + 'static>(&self) -> bool {
        self.0.is::<Box<T>>()
    }
    pub fn as_mut<T: ?Sized + 'static>(&mut self) -> Option<&mut Box<T>> {
        self.0.downcast_mut::<Box<T>>()
    }
}

pub trait InputListener
where
    Self: 'static,
{
    fn on_input(&mut self, event: &winit::event::WindowEvent);
}

pub trait Updateable
where
    Self: 'static,
{
    /// Can also render inside this function
    fn on_tick(&mut self, gl: &glow::Context, delta: &time::Duration, since_0: &time::Duration);
    fn on_setup(&mut self, gl: &glow::Context);
}

pub trait Collider
where
    Self: 'static,
{
    fn on_collide(&mut self, other: &dyn Collider);
    fn identifier(&self) -> RegisteredCollider;
    fn check_collision(&self, other: &dyn Collider);
}

// TODO: using macro to create these shits
// 1st degree Permutation of the basic traits ---------------------------------------------
pub trait UpdtInpLstr: Updateable + InputListener {}
impl<T: Updateable + InputListener> UpdtInpLstr for T {}

pub trait CldrUpdt : Collider + Updateable {}
impl<T: Collider + Updateable> CldrUpdt for T {}

pub trait CldrInpLstr : Collider + InputListener {}
impl<T: Collider + InputListener> CldrInpLstr for T {}

// 2nd degree permutation -----------------------------------------------------------
pub trait CldrUpdtInpLstr : Collider + Updateable + InputListener {}

impl<T: Collider + Updateable + InputListener> CldrUpdtInpLstr for T {}