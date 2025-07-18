use core::time;
use std::any::Any;

use crate::app::{board::Board, collider::ColliderType, App, ColliderLayer};

pub struct AppOwnedData(Box<dyn Any>);

trait AppOwnedDataTrait {}
macro_rules! impl_app_owned_data_trait {
    ($($ty:path),+) => {
        $(
            impl AppOwnedDataTrait for Box<dyn $ty>{}
        )+
    };
}

impl_app_owned_data_trait! {
    Updateable,
    Collider,
    InputListener,
    UpdtInpLstr,
    CldrUpdt,
    CldrInpLstr,
    CldrUpdtInpLstr
}

macro_rules! define_as {
    ($name:ident : $base:path where $($ty: path),+) => {
        pub fn $name(&mut self) -> Option<&mut dyn $base>{
            $(
                if self.is::<dyn $ty>(){
                    let data = self.as_mut::<dyn $ty>()?;
                    return Some(data.as_mut());
                };
            )+
            return None;
        }
    }
}
macro_rules! define_as_ref {
    ($name:ident : $base:path where $($ty: path),+) => {
        pub fn $name(& self) -> Option<& dyn $base>{
            $(
                if self.is::<dyn $ty>(){
                    let data = self.as_ref::<dyn $ty>()?;
                    return Some(data.as_ref());
                };
            )+
            return None;
        }
    }
}

impl AppOwnedData {
    /// # Warning
    /// data must be trait object of that implement `AppOwnedDataTrait` <br/>
    /// if an object implement more than one trait that implement `AppOwnedDataTrait`, use the permutation version, so it will be registered as both trait <br/>
    /// eg: `Collider + Updateable` use `CldrUpdt`
    #[allow(private_bounds)]
    pub fn from<T: AppOwnedDataTrait + 'static>(data: T) -> Self {
        AppOwnedData(Box::new(data))
    }
    
    define_as! {as_setupable : Setupable where Updateable, Collider, InputListener, UpdtInpLstr, CldrUpdt, CldrInpLstr, CldrUpdtInpLstr }
    define_as! {as_updateable : Updateable where Updateable, UpdtInpLstr, CldrUpdt, CldrUpdtInpLstr}
    define_as! {as_collider : Collider where Collider, CldrInpLstr, CldrUpdt, CldrUpdtInpLstr}
    define_as_ref! {as_ref_collider : Collider where Collider, CldrInpLstr, CldrUpdt, CldrUpdtInpLstr}
    define_as! {as_input_listener : InputListener where InputListener, CldrInpLstr, UpdtInpLstr, CldrUpdtInpLstr}

    fn is<T: ?Sized + 'static>(&self) -> bool {
        self.0.is::<Box<T>>()
    }
    pub fn as_mut<T: ?Sized + 'static>(&mut self) -> Option<&mut Box<T>> {
        self.0.downcast_mut::<Box<T>>()
    }
    pub fn as_ref<T: ?Sized + 'static>(&self) -> Option<&Box<T>> {
        self.0.downcast_ref::<Box<T>>()
    }
}

pub trait Setupable {
    fn on_setup(&mut self, gl: &glow::Context, registered_idx:usize, board: &Board);
}

pub trait InputListener : Setupable
where
    Self: 'static,
{
    fn on_input(&mut self, event: &winit::event::WindowEvent, board : &Board);
}

pub struct Time<'a> {
    pub delta: &'a time::Duration, 
    pub elapsed: &'a time::Duration,
}
pub trait Updateable : Setupable
where
    Self: 'static,
{
    /// Can also render inside this function
    fn on_tick(&mut self, gl: &glow::Context, time : &Time, board: &Board, game_over : &mut dyn FnMut());
}

pub trait Collider : Setupable
where
    Self: 'static,
{
    fn check_collision(&mut self, other: &dyn Collider);
    fn layer(&self) -> ColliderLayer;
    fn collider(&self) -> Vec<ColliderType>;
}

macro_rules! create_super_trait {
    ($name:ident : $($bounds:path),+) => {
        pub trait $name: $($bounds + )+ {}
        impl<T: $($bounds + )+> $name for T {}
    };
}

create_super_trait!(UpdtInpLstr: Updateable, InputListener);
create_super_trait!(CldrUpdt: Collider, Updateable);
create_super_trait!(CldrInpLstr: Collider, InputListener);
create_super_trait!(CldrUpdtInpLstr: Collider, Updateable, InputListener);
