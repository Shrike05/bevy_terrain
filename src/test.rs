use crate::*;
use bevy::prelude::*;
use bevy::render::{RenderPlugin, settings::WgpuSettings};
use bevy::winit::WinitPlugin;

#[test]
fn build_test() {
    // Setup app
    let mut app = App::new();

    // Add Score resource
    app.add_plugins(
        DefaultPlugins
            .set(RenderPlugin {
                // This tells Bevy to skip trying to talk to a real GPU
                render_creation: WgpuSettings {
                    backends: None,
                    ..default()
                }
                .into(),
                ..default()
            })
            .set(WinitPlugin {
                // This prevents Bevy from panicking because the test isn't on the "main" thread
                run_on_any_thread: true,
            }),
    );

    app.add_plugins(TerrainPlugin::new(4));

    app.run();
}
