use std::collections::HashSet;
use crate::demo::cell_bundle::{CellCreature, CellSize};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(detect_cell_collisions)
        .add_observer(clean_up_eaten_cells)
        .init_resource::<CollisionsAlreadyProcessedCache>()
        .register_type::<EatenBy>();
}

#[derive(Event, Reflect)]
pub struct EatenBy {
    pub eater: Entity,
}

#[derive(Resource, Default)]
struct CollisionsAlreadyProcessedCache {
    entities: HashSet<Entity>,
}

// System to detect collisions between cells
fn detect_cell_collisions(
    trigger: Trigger<OnCollisionStart>,
    mut commands: Commands,
    cells: Query<(Entity, &CellSize), With<CellCreature>>,
    mut cache: ResMut<CollisionsAlreadyProcessedCache>,
) {
    // Check if both entities have Cell components
    if let (Ok((e1, CellSize(s1))), Ok((e2, CellSize(s2)))) =
        (cells.get(trigger.target()), cells.get(trigger.collider))
    {
        // Determine which cell eats which (larger eats smaller)
        let (eater, eaten) = if s1 > s2 { (e1, e2) } else { (e2, e1) };

        if !cache.entities.contains(&eaten) && let Ok(mut ec) = commands.get_entity(eaten) {
            ec.trigger(EatenBy { eater });
            cache.entities.insert(eaten.clone());
        }
    }
}

fn clean_up_eaten_cells(eaten: Trigger<EatenBy>, mut commands: Commands, mut cache: ResMut<CollisionsAlreadyProcessedCache>) {
    let entity = eaten.target();
    if let Ok(mut ec) = commands.get_entity(entity) {
        ec.despawn();
        cache.entities.remove(&entity);
    }
}
