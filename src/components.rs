use std::any::Any;
use super::{Component, ComponentType};

pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Component for Position {
    fn get_component_type() -> ComponentType {
        ComponentType::of::<Self>()
    }

    // fn get_object_type(&self) -> ObjectType {
    //     Self::get_component_type()
    // }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct TestComponent;

impl Component for TestComponent {
    fn get_component_type() -> ComponentType {
        ComponentType::of::<Self>()
    }

    // fn get_object_type(&self) -> TypeId {
    //     Self::get_component_type()
    // }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct Input;

impl Component for Input {
    fn get_component_type() -> ComponentType {
        ComponentType::of::<Self>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}


// struct Velocity {
//     dx: i32,
//     dy: i32
// }

// impl Component for Velocity {
//     fn get_component_type(&self) -> ComponentType {
//         ComponentType::Velocity
//     }
// }

