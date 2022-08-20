use bevy::{prelude::*, render::texture::ImageSettings};

mod direction;
mod tilemap;

mod prelude {
    pub use super::direction::*;
    pub use super::AppState;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AppState {
    LoadingAssets,
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(tilemap::Plugin)
        .add_state(AppState::LoadingAssets)
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system(startup_system)
        .run();
}

fn startup_system(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
