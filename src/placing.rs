use crate::{prelude::*, tilemap::*};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(place_tiles_system));
    }
}

fn place_tiles_system(
    mut commands: Commands,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut tilemap: ResMut<Tilemap>,
) {
    let (camera, camera_transform) = camera_query.single();
    if mouse_buttons.just_released(MouseButton::Right) {
        if let Some(grid_pos) = cursor_to_grid(CursorToWorldInputs {
            windows: &windows,
            camera,
            camera_transform,
        }) {
            tilemap.add(
                grid_pos.tile,
                TileBuilder::Belt(Side::from_hitvec(grid_pos.hitvec)),
                &mut commands,
            );
        }
    }
}
