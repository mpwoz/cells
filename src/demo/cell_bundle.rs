use bevy::prelude::{Bundle, ChildOf, Component, Entity, Name, Transform};
use bevy::math::Vec2;
use avian2d::prelude::{Collider, CollisionEventsEnabled, RigidBody};
use crate::demo::movement::ScreenWrap;

#[derive(Component)]
pub struct CellCreature;

#[derive(Component)]
pub struct CellSize(pub f32);

#[derive(Bundle)]
pub struct CellBundle {
    name: Name,
    cell: CellCreature,
    cell_size: CellSize, // abstract "size" component that determines the cell's "strength"
    transform: Transform,
    screen_wrap: ScreenWrap,
    child_of: ChildOf,
    rigid_body: RigidBody,
    collider: Collider,
    collision_events_enabled: CollisionEventsEnabled,
}

impl CellBundle {
    pub(crate) fn new(name: &str, parent: Entity, position: Vec2, radius: f32) -> Self {
        Self {
            name: Name::new(name.to_string()),
            cell: CellCreature,
            cell_size: CellSize(radius),
            transform: Transform::from_translation(position.extend(0.0)),
            screen_wrap: ScreenWrap,
            child_of: ChildOf(parent),
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(radius),
            collision_events_enabled: CollisionEventsEnabled,
        }
    }
}