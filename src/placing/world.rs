use super::ToolDirection;
use crate::{prelude::*, tilemap::*};
use bevy::prelude::*;

const CURSOR_COLOR_OK: Color = Color::rgba(1.0, 1.0, 1.0, 0.5);
const CURSOR_COLOR_ERR: Color = Color::rgba(1.0, 0.0, 0.0, 0.5);

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_exit(AppState::LoadingAssets).with_system(setup_system))
            .add_system_set(
                SystemSet::on_update(AppState::Game)
                    .with_system(place_tiles_system)
                    .with_system(cursor_system),
            );
    }
}

#[derive(Debug, Component)]
struct Cursor {
    target: IVec2,
    is_visible: bool,
}

fn setup_system(mut commands: Commands, tilemap: Res<Tilemap>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: tilemap.textures().belt[Side::North].clone(),
                custom_size: Some(Vec2::ONE),
                color: Color::NONE,
                ..default()
            },
            texture_atlas: tilemap.atlas().clone(),
            ..default()
        })
        .insert(Cursor {
            target: IVec2::ZERO,
            is_visible: false,
        });
}

fn place_tiles_system(
    mut commands: Commands,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut tilemap: ResMut<Tilemap>,
    placing_direction: Res<ToolDirection>,
) {
    let (camera, camera_transform) = camera_query.single();
    let place_buttons = [MouseButton::Left, MouseButton::Right];
    if mouse_buttons.any_pressed(place_buttons) | mouse_buttons.any_just_pressed(place_buttons) {
        if let Some(grid_pos) = cursor_to_grid_pos(CursorToWorldInputs {
            windows: &windows,
            camera,
            camera_transform,
        }) {
            if tilemap.get(grid_pos.tile).is_none() {
                tilemap.add(
                    grid_pos.tile,
                    MachineType::Belt,
                    placing_direction.0,
                    &mut commands,
                );
            }
        }
    }
}

fn cursor_system(
    mut cursor_query: Query<(&mut Cursor, &mut Transform, &mut TextureAtlasSprite)>,
    placing_direction: Res<ToolDirection>,
    tilemap: Res<Tilemap>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    time: Res<Time>,
) {
    let (mut cursor, mut transform, mut sprite) = cursor_query.single_mut();

    let ideal_rotation = placing_direction.0.as_angle();
    transform.rotation = Quat::from_axis_angle(-Vec3::Z, ideal_rotation);

    let (camera, camera_transform) = camera_query.single();
    let ideal_position = cursor_to_grid_pos(CursorToWorldInputs {
        windows: &windows,
        camera,
        camera_transform,
    });
    if let Some(ideal_position) = ideal_position {
        cursor.target = ideal_position.tile;
        sprite.color = match tilemap.get(ideal_position.tile) {
            Some(_) => CURSOR_COLOR_ERR,
            None => CURSOR_COLOR_OK,
        };
        if !cursor.is_visible {
            cursor.is_visible = true;
            transform.translation = ideal_position.tile.as_vec2().extend(10.0);
        } else {
            let seperation = cursor.target.as_vec2().extend(10.0) - transform.translation;
            transform.translation += seperation * 8.0 * time.delta_seconds();
        }
    } else {
        sprite.color = Color::NONE;
        cursor.is_visible = false;
    }
}
