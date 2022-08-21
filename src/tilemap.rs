use crate::prelude::*;
use bevy::{prelude::*, utils::HashMap};

mod setup;
mod transformations;

pub const MIN_MAP_SIZE: usize = 16;

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
pub enum MachineType {
    Belt,
}

#[derive(Debug)]
pub enum Tile {
    Belt(Side, Entity),
}

#[derive(Debug)]
pub struct TextureMap {
    pub belt: SideArr<usize>,
    atlas: Handle<TextureAtlas>,
}

#[derive(Debug, Component)]
struct TileComponent;

impl Tilemap {
    /// Returns the texture atlas this takes textures from
    pub fn atlas(&self) -> &Handle<TextureAtlas> {
        &self.textures.atlas
    }

    /// Returns the texture map this is using
    pub fn textures(&self) -> &TextureMap {
        &self.textures
    }

    pub fn get(&self, tile: IVec2) -> Option<&Tile> {
        self.data.get(&tile)
    }

    /// Adds a tile to an empty tilemap square
    ///
    /// # Panics
    ///
    /// Panics if the square is occupied
    pub fn add(
        &mut self,
        pos: IVec2,
        tile: MachineType,
        facing_side: Side,
        commands: &mut Commands,
    ) {
        match tile {
            MachineType::Belt => {
                let entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: transform_from_grid_pos(pos, 4.0),
                        sprite: TextureAtlasSprite {
                            index: self.textures.belt[facing_side],
                            custom_size: Some(Vec2::ONE),
                            ..default()
                        },
                        texture_atlas: self.textures.atlas.clone(),
                        ..default()
                    })
                    .id();
                self.data
                    .try_insert(pos, Tile::Belt(facing_side, entity))
                    .unwrap();
            }
        }
    }
}

impl MachineType {
    /// The size of this machine in tiles
    pub fn size(self) -> UVec2 {
        use MachineType::*;
        match self {
            Belt => UVec2::ONE,
        }
    }
}
