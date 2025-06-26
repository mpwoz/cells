use bevy::prelude::{Bundle, ChildOf, Entity, Name, Transform};
use bevy::math::Vec2;
use avian2d::prelude::{Collider, RigidBody};
use crate::demo::movement::ScreenWrap;

#[derive(Bundle)]
pub struct CellBundle {
    name: Name,
    transform: Transform,
    screen_wrap: ScreenWrap,
    child_of: ChildOf,
    rigid_body: RigidBody,
    collider: Collider,
}

impl CellBundle {
    pub(crate) fn new(name: &str, parent: Entity, position: Vec2, radius: f32) -> Self {
        Self {
            name: Name::new(name.to_string()),
            transform: Transform::from_translation(position.extend(0.0)),
            screen_wrap: ScreenWrap,
            child_of: ChildOf(parent),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(radius),
        }
    }
}