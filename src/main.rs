use bevy::{prelude::*, render::texture::ImageSettings};
use tilemap::GridPos;

mod direction;
mod items;
mod placing;
mod tilemap;

mod prelude {
    pub use super::direction::*;
    pub use super::{AppState, MainCamera, MouseInput};
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum AppState {
    LoadingAssets,
    Game,
}

fn main() {
    App::new()
        .insert_resource(ImageSettings::default_nearest())
        .init_resource::<MouseInput>()
        .add_state(AppState::LoadingAssets)
        .add_plugins(DefaultPlugins)
        .add_plugin(items::Plugin)
        .add_plugin(placing::Plugin)
        .add_plugin(tilemap::Plugin)
        .add_startup_system(startup_system)
        .add_system_set_to_stage(
            CoreStage::PreUpdate,
            SystemSet::new().with_system(capture_mouse_input_system),
        )
        .run();
}

#[derive(Component)]
pub struct MainCamera;

fn startup_system(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.0125,
                ..default()
            },
            ..default()
        })
        .insert(MainCamera);
}

/// Inputs, captured in `CoreState::PreUpdate`
#[derive(Debug, Default)]
pub struct MouseInput {
    /// The grid position of the mouse
    pub pos: Option<GridPos>,
    /// Whether or not the mouse is clicked.
    /// Guaranteed to be false when `pos` is `None`.
    pub is_clicked: bool,
}

impl MouseInput {
    /// Returns `Some` if the pointer is over the window and clicked,
    /// otherwise returns `None`
    pub fn clicked_pos(&self) -> Option<GridPos> {
        self.pos.filter(|_| self.is_clicked)
    }
}

fn capture_mouse_input_system(
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    toolbar_query: Query<(&Node, &GlobalTransform), With<placing::toolbar::Background>>,
    mouse_buttons: Res<Input<MouseButton>>,
    state: Res<State<AppState>>,
    mut mouse_input: ResMut<MouseInput>,
) {
    use tilemap::*;

    if *state.current() != AppState::Game {
        return;
    }

    let window = windows.get_primary();

    let pos = window
        .and_then(|w| w.cursor_position())
        .filter(|mouse_pos| {
            let (node, transform) = toolbar_query.single();
            mouse_pos.y > transform.translation().y + node.size.y * 0.5
        })
        .map(|screen_pos| {
            let (camera, camera_transform) = camera_query.single();
            screen_to_grid_pos(
                ScreenToWorldInputs {
                    window: window.unwrap(),
                    camera,
                    camera_transform,
                },
                screen_pos,
            )
        });

    let place_buttons = [MouseButton::Left, MouseButton::Right];
    let is_clicked = pos.is_some()
        && (mouse_buttons.any_pressed(place_buttons)
            || mouse_buttons.any_just_pressed(place_buttons));

    *mouse_input = MouseInput { pos, is_clicked };
}
