use avian2d::prelude::*;
use bevy::prelude::*;
use crate::demo::cell_bundle::{CellCreature, CellSize};

pub fn plugin(app: &mut App) {
    app
        .add_observer(detect_cell_collisions)
        .register_type::<EatEvent>();
}

#[derive(Event, Reflect)]
pub struct EatEvent {
    eater: Entity,
    target: Entity,
}

// System to detect collisions between cells
fn detect_cell_collisions(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    cells: Query<(Entity, &CellSize), With<CellCreature>>,
) {
    // Check if both entities have Cell components
    if let (Ok((e1, CellSize(s1))), Ok((e2, CellSize(s2)))) = (
        cells.get(trigger.target()),
        cells.get(trigger.collider),
    ) {
        // Determine which cell eats which (larger eats smaller)
        let (eater, eaten) = if s1 > s2 {
            (e1, e2)
        } else {
            (e2, e1)
        };

        info!("Cell {:?} ate cell {:?}", eater, eaten);
        commands.entity(eaten).despawn();
    }
}