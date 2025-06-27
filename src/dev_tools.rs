//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((perf_ui::plugin, world_inspector::plugin));

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

mod perf_ui {
    use bevy::prelude::*;
    use iyes_perf_ui::entries::{PerfUiFramerateEntries, PerfUiWindowEntries};
    use iyes_perf_ui::prelude::*;

    pub fn plugin(app: &mut App) {
        // we want Bevy to measure these values for us:
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
            .add_plugins(PerfUiPlugin);

        app.add_systems(Startup, setup);
    }
    #[derive(Component)]
    pub struct PerfUiMarker;

    pub fn setup(mut commands: Commands) {
        // create a simple Perf UI with default settings
        // and all entries provided by the crate:
        commands.spawn((
            Name::from("PerfUi"),
            PerfUiMarker,
            PerfUiRoot {
                position: PerfUiPosition::TopRight,
                ..default()
            },
            // Contains everything related to FPS and frame time
            PerfUiFramerateEntries::default(),
            // Contains everything related to the window and cursor
            PerfUiWindowEntries::default(),
            // Contains everything related to system diagnostics (CPU, RAM)
            // PerfUiSystemEntries::default(),
        ));
    }
}

mod world_inspector {

    use bevy::prelude::*;
    use bevy_inspector_egui::bevy_egui::EguiPlugin;
    use bevy_inspector_egui::quick::WorldInspectorPlugin;

    pub fn plugin(app: &mut App) {
        app.add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        });
        app.add_plugins(WorldInspectorPlugin::default());
    }
}
