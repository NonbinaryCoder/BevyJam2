use crate::items::Item;
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
    Ice,
    Combiner2x1,
}

#[derive(Debug)]
pub enum Tile {
    Belt(Side, Entity),
    Ice(Entity),
    CombinerInput(CombinerInput),
    Combiner2x1(Box<Combiner2x1>),
}

#[derive(Debug)]
pub struct CombinerInput {
    input_side: Side,
    parent: IVec2,
}

impl From<CombinerInput> for Tile {
    fn from(f: CombinerInput) -> Self {
        Tile::CombinerInput(f)
    }
}

#[derive(Debug)]
pub struct Combiner2x1 {
    input_side: Side,
    inputs: [Option<Item>; 2],
    entity: Entity,
}

impl From<Combiner2x1> for Tile {
    fn from(f: Combiner2x1) -> Self {
        Tile::Combiner2x1(Box::new(f))
    }
}

#[derive(Debug)]
pub struct TextureMap {
    pub delete_tool: usize,
    pub belt: usize,
    pub ice: usize,
    pub combiner2x1: usize,
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
    #[must_use]
    pub fn atlas(&self) -> &Handle<TextureAtlas> {
        &self.textures.atlas
    }

    /// Returns the texture map this is using
    #[must_use]
    pub fn textures(&self) -> &TextureMap {
        &self.textures
    }

    #[must_use]
    pub fn get_tile(&self, tile: IVec2) -> Option<&Tile> {
        self.data.get(&tile)
    }

    #[must_use]
    pub fn is_area_clear(&self, cursor_position: IVec2, size: UVec2, offset: IVec2) -> bool {
        let min = cursor_position + offset;
        let max = size.as_ivec2() + offset + cursor_position;
        let is_not_clear = (min.x..max.x)
            .flat_map(|x| (min.y..max.y).map(move |y| IVec2::new(x, y)))
            .any(|key| self.data.contains_key(&key));
        !is_not_clear
    }

    /// Adds a tile to the tilemap if there is space for it
    pub fn try_add(
        &mut self,
        pos: IVec2,
        tile: MachineType,
        facing_side: Side,
        commands: &mut Commands,
    ) {
        let mut spawn_rect = |index, z, size, offset: Vec2| {
            let mut transform = transform_from_grid_pos(pos, z, facing_side);

            transform.translation += offset.extend(0.0);
            commands
                .spawn_bundle(SpriteSheetBundle {
                    transform,
                    sprite: TextureAtlasSprite {
                        index,
                        custom_size: Some(size),
                        ..default()
                    },
                    texture_atlas: self.textures.atlas.clone(),
                    ..default()
                })
                .id()
        };
        let mut spawn_square = |index, z| spawn_rect(index, z, Vec2::ONE, Vec2::ZERO);
        match tile {
            MachineType::Belt => {
                self.data.entry(pos).or_insert_with(|| {
                    let entity = spawn_square(self.textures.belt, 2.0);
                    Tile::Belt(facing_side, entity)
                });
            }
            MachineType::Ice => {
                self.data.entry(pos).or_insert_with(|| {
                    let entity = spawn_square(self.textures.ice, 2.0);
                    Tile::Ice(entity)
                });
            }
            MachineType::Combiner2x1 => {
                let input_side = facing_side.opposite();
                if !self.data.contains_key(&pos)
                    && !self
                        .data
                        .contains_key(&(pos + input_side.rotate_right().to_ivec2()))
                {
                    let entity = spawn_rect(
                        self.textures.combiner2x1,
                        4.0,
                        Vec2::new(2.0, 1.0),
                        facing_side.rotate_vec2(MachineType::Combiner2x1.cursor_offset()),
                    );
                    self.data.insert(
                        pos,
                        Combiner2x1 {
                            input_side,
                            inputs: [None, None],
                            entity,
                        }
                        .into(),
                    );
                    self.data.insert(
                        pos + input_side.rotate_right().to_ivec2(),
                        CombinerInput {
                            input_side,
                            parent: pos,
                        }
                        .into(),
                    );
                }
            }
        }
    }

    /// Removes a tile from the tilemap
    pub fn remove(&mut self, pos: IVec2, commands: &mut Commands) {
        match self.data.remove(&pos) {
            None => return,
            Some(Tile::Belt(_, entity)) | Some(Tile::Ice(entity)) => {
                commands.entity(entity).despawn();
            }
            Some(Tile::CombinerInput(c)) => self.remove(c.parent, commands),
            Some(Tile::Combiner2x1(c)) => {
                commands.entity(c.entity).despawn();
                self.data
                    .remove(&(pos + c.input_side.rotate_right().to_ivec2()));
            }
        }
        if self.data.capacity() > MIN_MAP_SIZE.max(self.data.len()) {
            self.data.shrink_to(MIN_MAP_SIZE.max(self.data.len() + 8));
        }
    }
}

impl MachineType {
    /// The size of this machine in tiles
    #[must_use]
    pub fn size(self) -> UVec2 {
        use MachineType::*;
        match self {
            Belt | Ice => UVec2::ONE,
            Combiner2x1 => UVec2::new(2, 1),
        }
    }

    /// Offset of the center of this sprite from it's grid position
    #[must_use]
    pub fn cursor_offset(self) -> Vec2 {
        use MachineType::*;
        match self {
            Combiner2x1 => Vec2::new(-0.5, 0.0),
            _ => Vec2::ZERO,
        }
    }

    /// The offset of the northwest corner of the grid size of this from it's grid position
    #[must_use]
    pub fn grid_offset(self) -> IVec2 {
        use MachineType::*;
        match self {
            Combiner2x1 => IVec2::new(-1, 0),
            _ => IVec2::ZERO,
        }
    }
}
