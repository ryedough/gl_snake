use core::time;
use std::any::Any;

use crate::app::RegisteredCollider;

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

impl AppOwnedData {
    /// # Warning
    /// data must be trait object of that implement `AppOwnedDataTrait` <br/>
    /// if an object implement more than one trait that implement `AppOwnedDataTrait`, use the permutation version, so it will be registered as both trait <br/>
    /// eg: `Collider + Updateable` use `CldrUpdt`
    #[allow(private_bounds)]
    pub fn from<T: AppOwnedDataTrait + 'static>(data: T) -> Self {
        AppOwnedData(Box::new(data))
    }
    define_as! {as_updateable : Updateable where Updateable, UpdtInpLstr, CldrUpdt, CldrUpdtInpLstr}
    define_as! {as_collider : Collider where Collider, CldrInpLstr, CldrUpdt, CldrUpdtInpLstr}
    define_as! {as_input_listener : InputListener where InputListener, CldrInpLstr, UpdtInpLstr, CldrUpdtInpLstr}

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
