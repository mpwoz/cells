//! Demo gameplay. All of these modules are only intended for demonstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

mod animation;
mod cell_bundle;
mod eating;
mod enemy;
pub mod level;
mod mouse_position;
mod movement;
pub mod player;
mod stats;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        enemy::plugin,
        mouse_position::plugin,
        eating::plugin,
        stats::plugin,
    ));
}
