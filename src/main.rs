use bevy::{prelude::*, render::texture::ImageSettings};

mod direction;
mod placing;
mod tilemap;

mod prelude {
    pub use super::direction::*;
    pub use super::AppState;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AppState {
    LoadingAssets,
    Game,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .add_plugins(DefaultPlugins)
        .add_plugin(tilemap::Plugin)
        .add_plugin(placing::Plugin)
        .add_state(AppState::LoadingAssets)
        .add_startup_system(startup_system)
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.0125,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        ..default()
    });
}
