//! Player-specific behavior.

use avian2d::prelude::{LinearVelocity, RigidBody};
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};

use crate::demo::cell_bundle::CellBundle;
use crate::demo::mouse_position::MousePosition;
use crate::demo::stats::StatsTracker;
use crate::{
    AppSystems, PausableSystems, asset_tracking::LoadResource, demo::movement::MovementController,
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((SpawnPlayerIntoLevel::plugin,));

    app.register_type::<Player>();

    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input
            .in_set(AppSystems::RecordInput)
            .in_set(PausableSystems),
    );
}

#[derive(Event, Reflect)]
pub(crate) struct SpawnPlayerIntoLevel;

impl SpawnPlayerIntoLevel {
    pub fn plugin(app: &mut App) {
        app.add_observer(Self::observer).register_type::<Self>();
    }
    pub fn observer(
        trigger: Trigger<SpawnPlayerIntoLevel>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let radius = 25.0;
        let shape = meshes.add(Circle::new(radius));
        let default_material = materials.add(Color::srgb(1., 1., 1.));

        let level_entity = trigger.target();
        commands.spawn((
            Player,
            CellBundle::new("Player", level_entity, Vec2::ZERO, radius),
            Mesh2d(shape.clone()),
            MeshMaterial2d(default_material.clone()),
            MovementController { ..default() },
            StatsTracker::default(),
        ));
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;

fn record_player_directional_input(
    mut controller_query: Query<(&mut MovementController, &Transform), With<Player>>,
    mouse_position: Res<MousePosition>,
) {
    // Apply movement intent to controllers.
    for (mut controller, t) in &mut controller_query {
        let intent = (mouse_position.0 - t.translation.xy()).normalize();
        controller.intent = intent;
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky: Handle<Image>,
    #[dependency]
    pub steps: Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            ducky: assets.load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),
            steps: vec![
                assets.load("audio/sound_effects/step1.ogg"),
                assets.load("audio/sound_effects/step2.ogg"),
                assets.load("audio/sound_effects/step3.ogg"),
                assets.load("audio/sound_effects/step4.ogg"),
            ],
        }
    }
}
