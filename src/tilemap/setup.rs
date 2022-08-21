use super::{TextureMap, Tilemap};
use crate::prelude::*;
use bevy::{asset::LoadState, prelude::*, utils::HashMap};

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system).add_system_set(
            SystemSet::on_update(AppState::LoadingAssets).with_system(create_atlas_system),
        );
    }
}

#[derive(Debug)]
struct TileTextureHandles(Vec<HandleUntyped>);

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TileTextureHandles(
        asset_server.load_folder("tiles").unwrap(),
    ));
}

fn create_atlas_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    handles: Res<TileTextureHandles>,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    match asset_server.get_group_load_state(handles.0.iter().map(|h| h.id)) {
        LoadState::NotLoaded | LoadState::Loading => (),
        LoadState::Failed => panic!("Failed to load tile textures!"),
        LoadState::Unloaded => unreachable!(),
        LoadState::Loaded => {
            let atlas = {
                let mut atlas_builder = TextureAtlasBuilder::default();
                for handle in handles.0.iter() {
                    let handle = handle.typed_weak();
                    let texture = textures.get_mut(&handle).unwrap();
                    atlas_builder.add_texture(handle, texture);
                }
                atlas_builder.finish(&mut textures).unwrap()
            };

            let handle_from_name = |name| {
                atlas
                    .get_texture_index(&asset_server.get_handle(name))
                    .unwrap_or_else(|| panic!("Missing texture: \"{name}\""))
            };

            let texture_map = TextureMap {
                belt: SideArr {
                    north: handle_from_name("tiles/belt_north_0.png"),
                    east: handle_from_name("tiles/belt_east_0.png"),
                    south: handle_from_name("tiles/belt_south_0.png"),
                    west: handle_from_name("tiles/belt_west_0.png"),
                },
                atlas: atlases.add(atlas),
            };

            commands.insert_resource(Tilemap {
                data: HashMap::with_capacity(super::MIN_MAP_SIZE),
                textures: texture_map,
            });

            commands.remove_resource::<TileTextureHandles>();
            state.set(AppState::Game).unwrap();
        }
    }
}
