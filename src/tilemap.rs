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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachineType {
    Belt,
}

#[derive(Debug)]
pub enum Tile {
    Belt(Side, Entity),
}

#[derive(Debug)]
pub struct TextureMap {
    pub delete_tool: usize,
    pub belt: usize,
    pub item_a: usize,
    pub item_b: usize,
    pub item_c: usize,
    pub item_d: usize,
    pub atlas: Handle<TextureAtlas>,
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

    pub fn get_tile(&self, tile: IVec2) -> Option<&Tile> {
        self.data.get(&tile)
    }

    /// Adds a tile to the tilemap if there is space for it
    pub fn try_add(
        &mut self,
        pos: IVec2,
        tile: MachineType,
        facing_side: Side,
        commands: &mut Commands,
    ) {
        match tile {
            MachineType::Belt => self.data.entry(pos).or_insert_with(|| {
                let entity = commands
                    .spawn_bundle(SpriteSheetBundle {
                        transform: transform_from_grid_pos(pos, 4.0, facing_side),
                        sprite: TextureAtlasSprite {
                            index: self.textures.belt,
                            custom_size: Some(Vec2::ONE),
                            ..default()
                        },
                        texture_atlas: self.textures.atlas.clone(),
                        ..default()
                    })
                    .id();
                Tile::Belt(facing_side, entity)
            }),
        };
    }

    /// Removes a tile from the tilemap
    pub fn remove(&mut self, pos: IVec2, commands: &mut Commands) {
        match self.data.remove(&pos) {
            None => return,
            Some(Tile::Belt(_, entity)) => commands.entity(entity).despawn(),
        }
        if self.data.capacity() > MIN_MAP_SIZE.max(self.data.len()) {
            self.data.shrink_to(MIN_MAP_SIZE.max(self.data.len() + 8));
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
