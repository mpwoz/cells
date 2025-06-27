use crate::demo::cell_bundle::{CellCreature, CellSize};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app
        .add_observer(detect_cell_collisions)
        .add_systems(Update, clean_up_eaten_cells)
        .register_type::<EatenBy>();
}

#[derive(Component, Reflect)]
pub struct EatenBy {
    eater: Entity,
}

// System to detect collisions between cells
fn detect_cell_collisions(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    cells: Query<(Entity, &CellSize), With<CellCreature>>,
) {
    // Check if both entities have Cell components
    if let (Ok((e1, CellSize(s1))), Ok((e2, CellSize(s2)))) =
        (cells.get(trigger.target()), cells.get(trigger.collider))
    {
        // Determine which cell eats which (larger eats smaller)
        let (eater, eaten) = if s1 > s2 { (e1, e2) } else { (e2, e1) };

        if let Ok(mut ec) = commands.get_entity(eaten) {
            ec.insert(EatenBy { eater });
        }
    }
}

fn clean_up_eaten_cells(eaten: Query<Entity, With<EatenBy>>, mut commands: Commands) {
    for entity in eaten.iter() {
        if let Ok(mut ec) = commands.get_entity(entity) {
            ec.despawn();
        }
    }
}
