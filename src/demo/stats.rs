use crate::demo::eating::EatenBy;
use crate::demo::player::Player;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_observer(update_stats_on_kill)
        .register_type::<StatsTracker>();
}
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct StatsTracker {
    kills: usize,
}

fn update_stats_on_kill(trigger: Trigger<EatenBy>, mut players: Query<&mut StatsTracker>) {
    if let Ok(mut p) = players.get_mut(trigger.eater) {
        p.kills += 1;
        info!("{:?} has {:?} kills", trigger.target(), p.kills);
    }
}
