use crate::prelude::*;
use bevy::prelude::*;

mod world;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(world::Plugin)
            .init_resource::<ToolDirection>()
            .add_system_set(
                SystemSet::on_update(AppState::Game).with_system(change_placing_direction_system),
            );
    }
}

#[derive(Debug, Default)]
pub struct ToolDirection(Side);

fn change_placing_direction_system(
    mut placing_direction: ResMut<ToolDirection>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.any_just_released([KeyCode::A, KeyCode::Left]) {
        placing_direction.0 = placing_direction.0.rotate_left();
    }

    if keys.any_just_released([KeyCode::D, KeyCode::Right]) {
        placing_direction.0 = placing_direction.0.rotate_right();
    }
}
