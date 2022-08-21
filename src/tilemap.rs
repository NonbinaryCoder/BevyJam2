use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};

mod setup;
mod transformations;

pub use transformations::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(setup::Plugin);
    }
}

#[derive(Debug)]
pub struct Tilemap {
    data: HashMap<IVec2, Tile>,
    textures: TextureMap,
}

#[derive(Debug)]
pub enum TileBuilder {
    Belt(Side),
}

#[derive(Debug)]
pub enum Tile {
    Belt(Side, Entity),
}

#[derive(Debug)]
pub struct TextureMap {
    belt: SideArr<usize>,
    atlas: Handle<TextureAtlas>,
}

#[derive(Debug, Component)]
struct TileComponent;

impl Tilemap {
    pub fn add(&mut self, pos: IVec2, tile: TileBuilder, commands: &mut Commands) {
        match tile {
            TileBuilder::Belt(side) => {
                let entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: transform_from_grid_pos(pos, 4.0),
                        sprite: TextureAtlasSprite {
                            index: self.textures.belt[side],
                            custom_size: Some(Vec2::ONE),
                            ..default()
                        },
                        texture_atlas: self.textures.atlas.clone(),
                        ..default()
                    })
                    .id();
                self.data.try_insert(pos, Tile::Belt(side, entity)).unwrap();
            }
        }
    }
}
