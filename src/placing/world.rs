use super::{Tool, ToolDirection};
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
                    .with_system(use_tool_system)
                    .with_system(cursor_system),
            );
    }
}

#[derive(Debug, Component)]
pub struct Cursor {
    target: IVec2,
    is_visible: bool,
}

fn setup_system(mut commands: Commands, tilemap: Res<Tilemap>) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: tilemap.textures().delete_tool.clone(),
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

fn use_tool_system(
    mut commands: Commands,
    mouse_input: Res<MouseInput>,
    mut tilemap: ResMut<Tilemap>,
    tool: Res<Tool>,
    placing_direction: Res<ToolDirection>,
) {
    if let Some(pos) = mouse_input.clicked_pos() {
        match &*tool {
            Tool::Delete => tilemap.remove(pos.tile, &mut commands),
            Tool::Place(machine_type) => {
                tilemap.try_add(pos.tile, *machine_type, placing_direction.0, &mut commands);
            }
        }
    }
}

fn cursor_system(
    mut cursor_query: Query<(&mut Cursor, &mut Transform, &mut TextureAtlasSprite)>,
    placing_direction: Res<ToolDirection>,
    tilemap: Res<Tilemap>,
    mouse_input: Res<MouseInput>,
    time: Res<Time>,
) {
    let (mut cursor, mut transform, mut sprite) = cursor_query.single_mut();

    let ideal_rotation = placing_direction.0.to_angle();
    transform.rotation = Quat::from_rotation_z(ideal_rotation);

    if let Some(ideal_position) = mouse_input.pos {
        cursor.target = ideal_position.tile;
        sprite.color = match tilemap.get_tile(ideal_position.tile) {
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
