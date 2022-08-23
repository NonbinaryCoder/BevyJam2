use crate::{prelude::*, tilemap::*};
use bevy::{math::Vec3Swizzles, prelude::*};

const BELT_SPEED: f32 = 1.0;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(item_momentum_system)
                .with_system(temp_spawn_items_system),
        );
    }
}

#[derive(Debug, Component)]
enum Item {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Component, Default, Clone, Copy)]
struct Momentum(Vec2);

fn temp_spawn_items_system(
    mut commands: Commands,
    mouse_input: Res<MouseInput>,
    keys: Res<Input<KeyCode>>,
    tilemap: Res<Tilemap>,
    items_query: Query<Entity, With<Item>>,
) {
    if let Some(pos) = mouse_input
        .pos
        .filter(|_| keys.just_pressed(KeyCode::Space))
    {
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: tilemap.textures().item_a.clone(),
                    custom_size: Some(Vec2::splat(0.5)),
                    ..default()
                },
                texture_atlas: tilemap.atlas().clone(),
                transform: transform_from_grid_pos(pos.tile, 6.0, Side::North),
                ..default()
            })
            .insert(Momentum::default())
            .insert(Item::A);
    }

    if keys.just_pressed(KeyCode::LShift) {
        for item in items_query.iter() {
            commands.entity(item).despawn();
        }
    }
}

fn item_momentum_system(
    mut items_query: Query<(&mut Transform, &mut Momentum), With<Item>>,
    tilemap: Res<Tilemap>,
    time: Res<Time>,
) {
    for (mut transform, mut momentum) in items_query.iter_mut() {
        match tilemap.get_tile(world_to_grid_pos(transform.translation.xy()).tile) {
            None => momentum.0 = Vec2::ZERO,
            Some(Tile::Belt(side, _)) => {
                macro_rules! update_momentum {
                    ($main:ident, $cross:ident) => {
                        momentum.0.$cross = 0.0;
                        let seperation = pos_fract(transform.translation.$cross + 0.5) - 0.5;
                        let dist = seperation.abs();
                        transform.translation.$cross -=
                            seperation.signum() * dist.min(time.delta_seconds()) * BELT_SPEED;
                        if dist < f32::EPSILON {
                            momentum.0.$main = side.to_vec2().$main * BELT_SPEED;
                        }
                    };
                }
                match side.axis() {
                    Axis2d::X => {
                        update_momentum!(x, y);
                    }
                    Axis2d::Y => {
                        update_momentum!(y, x);
                    }
                }
            }
        }

        transform.translation += (momentum.0 * time.delta_seconds()).extend(0.0);
    }
}

fn pos_fract(x: f32) -> f32 {
    (x.fract() + 1.0).fract()
}
