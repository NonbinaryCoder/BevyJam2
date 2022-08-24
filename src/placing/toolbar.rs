use super::Tool;
use crate::{prelude::*, tilemap::*};
use bevy::prelude::*;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_system))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(change_tool_system));
    }
}

#[derive(Component)]
struct ToolIcon {
    tool: Tool,
}

#[derive(Component)]
pub struct Background;

const DESELECTED_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
const SELECTED_COLOR: Color = Color::WHITE;

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    top: Val::Undefined,
                },
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            color: Color::BLACK.into(),
            ..default()
        })
        .insert(Background)
        .with_children(|toolbar| {
            use MachineType::*;
            use Tool::*;

            for (index, (tool, image)) in [
                (Delete, asset_server.load("ui/delete.png")),
                (Place(Belt), asset_server.load("tiles/belt_0.png")),
                (Place(Ice), asset_server.load("tiles/ice.png")),
                (
                    Place(Combiner2x1),
                    asset_server.load("tiles/combiner2x1.png"),
                ),
            ]
            .into_iter()
            .enumerate()
            {
                let size = tool.size().as_vec2();
                let aspect = size.y / size.x;
                let size = Size::new(Val::Px(50.0), Val::Px(50.0 * aspect));
                toolbar
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size,
                            margin: UiRect::all(Val::Px(5.0)),
                            ..default()
                        },
                        image: image.into(),
                        color: match index {
                            0 => SELECTED_COLOR,
                            _ => DESELECTED_COLOR,
                        }
                        .into(),
                        ..default()
                    })
                    .insert(ToolIcon { tool });
            }
        });
}

impl Tool {
    fn icon(&self, textures: &TextureMap) -> usize {
        match self {
            Tool::Delete => textures.delete_tool,
            Tool::Place(MachineType::Belt) => textures.belt,
            Tool::Place(MachineType::Ice) => textures.ice,
            Tool::Place(MachineType::Combiner2x1) => textures.combiner2x1,
        }
    }
}

fn change_tool_system(
    interaction_query: Query<(&Interaction, Entity), (With<ToolIcon>, Changed<Interaction>)>,
    mut icon_query: Query<(&ToolIcon, &mut UiColor)>,
    mut selected_tool: ResMut<Tool>,
    mut cursor_query: Query<(&mut TextureAtlasSprite, &mut super::world::Cursor)>,
    tilemap: Res<Tilemap>,
) {
    if let Some(new_tool) =
        interaction_query
            .iter()
            .find_map(|(interaction, entity)| match interaction {
                Interaction::Clicked => Some(entity),
                _ => None,
            })
    {
        for (_, mut color) in icon_query.iter_mut() {
            color.0 = DESELECTED_COLOR;
        }
        let (&ToolIcon { tool }, mut color) = icon_query.get_mut(new_tool).unwrap();
        color.0 = SELECTED_COLOR;
        *selected_tool = tool;
        let (mut cursor_sprite, mut cursor) = cursor_query.single_mut();
        cursor_sprite.index = tool.icon(tilemap.textures());
        cursor_sprite.custom_size = Some(tool.size().as_vec2());
        cursor.offset = tool.cursor_offset();
    }
}
