use crate::demo::cell_bundle::CellBundle;
use crate::demo::movement::MovementController;
use bevy::color::palettes;
use bevy::prelude::*;
use rand::random;
use rand_distr::{Distribution, Pareto};

pub fn plugin(app: &mut App) {
    app.add_plugins(SpawnEnemyIntoLevel::plugin)
        .init_resource::<EnemyMaterials>()
        .register_type::<EnemyMaterials>()
        .register_type::<Enemy>();
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Enemy;

impl Enemy {}

#[derive(Event, Reflect)]
pub(crate) struct SpawnEnemyIntoLevel;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct EnemyMaterials {
    prey: Handle<ColorMaterial>,
    predator: Handle<ColorMaterial>,
}
impl FromWorld for EnemyMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.resource_mut::<Assets<ColorMaterial>>();

        let predator = materials.add(Color::from(palettes::css::INDIAN_RED));
        let prey = materials.add(Color::from(palettes::css::MEDIUM_SEA_GREEN));

        Self { prey, predator }
    }
}

impl SpawnEnemyIntoLevel {
    pub fn plugin(app: &mut App) {
        app.add_observer(Self::observer).register_type::<Self>();
    }
    pub fn observer(
        trigger: Trigger<SpawnEnemyIntoLevel>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        enemy_materials: Res<EnemyMaterials>,
    ) {
        // random radius with bias towards smaller values
        let (min_radius, max_radius) = (5., 100.);
        let pareto: Pareto<f32> = Pareto::new(min_radius, 1.0).unwrap();
        let mut rng = rand::thread_rng();
        let radius = pareto.sample(&mut rng).min(max_radius);

        // random position
        let (min_range, max_range) = (200., 2200.);
        let pos = Self::random_position(min_range + radius, max_range - radius);

        let shape = meshes.add(Circle::new(radius));

        let level_entity = trigger.target();
        commands.spawn((
            Enemy,
            CellBundle::new("Enemy", level_entity, pos, radius),
            Mesh2d(shape.clone()),
            MeshMaterial2d(enemy_materials.prey.clone()),
            MovementController { ..default() },
        ));
    }

    fn random_position(min_range: f32, max_range: f32) -> Vec2 {
        let angle = random::<f32>() * 2. * std::f32::consts::PI;
        let distance = random::<f32>() * (max_range - min_range) + min_range;
        Vec2::new(angle.cos(), angle.sin()) * distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_position() {
        for i in 0..1000 {
            let min_range = 100.;
            let p = SpawnEnemyIntoLevel::random_position(min_range, min_range + 10.);
            assert!(
                p.length() > min_range,
                "{:?} was within the safe zone {:?}",
                p,
                p.length()
            );
        }
    }
}
