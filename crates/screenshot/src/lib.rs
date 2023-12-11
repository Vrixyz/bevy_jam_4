use std::path::PathBuf;

use bevy::{
    prelude::*, render::view::screenshot::ScreenshotManager, ui::widget::UiImageSize,
    window::PrimaryWindow,
};

pub struct ScreenshotPlugin;

impl Plugin for ScreenshotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_info_msg)
            .add_systems(Update, screenshot_system);
    }
}

// REF https://bevyengine.org/examples/Window/screenshot/
fn screenshot_system(
    input: Res<Input<KeyCode>>,
    main_window: Query<Entity, With<PrimaryWindow>>,
    mut screenshot_manager: ResMut<ScreenshotManager>,
    mut counter: Local<u32>,
) {
    // the reason why i didn't use if cfg!() is because
    // import for `rfd` is undeclared when done so
    // that's why only viable solution is to use
    // `get_path` function.

    // the reason why `get_path` is a closure is because
    // you cannot move `counter` inside a regular function

    #[cfg(target_arch = "wasm32")]
    let get_path = || Some(PathBuf::from(format!("./screenshot{}.png", *counter)));

    #[cfg(not(target_arch = "wasm32"))]
    let get_path = || {
        use rfd::FileDialog;
        let homedir = simple_home_dir::home_dir().unwrap_or_else(|| {
            log::warn!("Failed to get home directory.");
            PathBuf::new()
        });
        FileDialog::new()
            .set_directory(homedir)
            .set_file_name(format!("screenshot{}.png", *counter))
            .save_file()
    };

    if input.just_pressed(KeyCode::Space) {
        let path = get_path();
        *counter += 1;
        match path {
            Some(path) => {
                screenshot_manager
                    .save_screenshot_to_disk(main_window.single(), path)
                    .unwrap();
            }
            None => {
                log::warn!("Save path does not exist.");
            }
        }
    }
}

fn init_info_msg(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Spacebar to screenshot",
                TextStyle {
                    font_size: 16.0,
                    ..default()
                },
            ));
        });
}
