use std::path::PathBuf;

use bevy::{prelude::*, render::view::screenshot::ScreenshotManager, window::PrimaryWindow};

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, screenshot_system);
    }
}

// REF https://bevyengine.org/examples/Window/screenshot/
fn screenshot_system(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    if input.just_pressed(KeyCode::Space) {
        let path = {
            if cfg!(target_arch = "wasm32") {
                format!("./screenshot{}.png", *counter)
            } else {
                let homedir = simple_home_dir::home_dir().unwrap_or_else(|| {
                    log::warn!("Failed to get home directory.");
                    PathBuf::new()
                });
                format!("{}/screenshot{}.png", homedir.to_string_lossy(), *counter)
            }
        };
        *counter += 1;
        screenshot_manager
            .save_screenshot_to_disk(main_window.single(), path)
            .unwrap();
    }
}
