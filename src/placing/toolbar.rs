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
            for (index, (tool, image)) in [
                (Tool::Delete, asset_server.load("ui/delete.png")),
                (
                    Tool::Place(MachineType::Belt),
                    asset_server.load("tiles/belt_0.png"),
                ),
            ]
            .into_iter()
            .enumerate()
            {
                toolbar
                    .spawn_bundle(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(50.0), Val::Px(50.0)),
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
        }
    }
}

fn change_tool_system(
    interaction_query: Query<(&Interaction, Entity), (With<ToolIcon>, Changed<Interaction>)>,
    mut icon_query: Query<(&ToolIcon, &mut UiColor)>,
    mut selected_tool: ResMut<Tool>,
    mut cursor_query: Query<&mut TextureAtlasSprite, With<super::world::Cursor>>,
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
        let (tool, mut color) = icon_query.get_mut(new_tool).unwrap();
        color.0 = SELECTED_COLOR;
        *selected_tool = tool.tool;
        cursor_query.single_mut().index = tool.tool.icon(tilemap.textures());
    }
}
