use crate::demo::cell_bundle::CellBundle;
use crate::demo::movement::MovementController;
use bevy::prelude::*;
use rand::{random, Rng};
use rand_distr::{LogNormal, Distribution, Pareto};

pub fn plugin(app: &mut App) {
    app.add_plugins(SpawnEnemyIntoLevel::plugin)
        .register_type::<Enemy>();
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

impl Enemy {}

#[derive(Event, Reflect)]
pub(crate) struct SpawnEnemyIntoLevel;

impl SpawnEnemyIntoLevel {
    pub fn plugin(app: &mut App) {
        app.add_observer(Self::observer).register_type::<Self>();
    }
    pub fn observer(
        trigger: Trigger<SpawnEnemyIntoLevel>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        
        // random radius with bias towards smaller values
        let (min_radius, max_radius) = (5., 100.);
        // Pareto with α = 1.0 gives P(x) ∝ 1/x
        let pareto:Pareto<f32> = Pareto::new(min_radius, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        let radius = pareto.sample(&mut rng).min(max_radius);

        // random position
        let range = 1000.;
        let vec = Vec2::from(random::<(f32, f32)>());
        let pos = vec * range;
        
        let shape = meshes.add(Circle::new(radius));
        let default_material = materials.add(Color::srgb(1., 0.5, 0.5));

        let level_entity = trigger.target();
        commands.spawn((
            Enemy,
            CellBundle::new("Enemy", level_entity, pos, radius),
            Mesh2d(shape.clone()),
            MeshMaterial2d(default_material.clone()),
            MovementController { ..default() },
        ));
    }
}
