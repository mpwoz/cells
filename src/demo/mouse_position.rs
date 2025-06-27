use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn plugin(app: &mut App) {
    app.init_resource::<MousePosition>();
    app.add_systems(PreUpdate, my_cursor_system);
    app.register_type::<MousePosition>();
}

/// We will store the world position of the mouse cursor here.
#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct MousePosition(pub Vec2);

fn my_cursor_system(
    mut mycoords: ResMut<MousePosition>,
    q_window: Single<&Window, With<PrimaryWindow>>,
    q_camera: Single<(&Camera, &GlobalTransform)>,
) -> Result<()> {
    let window = q_window.into_inner();
    let (camera, camera_transform) = q_camera.into_inner();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        mycoords.0 = world_position;
    }

    Ok(())
}
