use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ScreenToWorldInputs<'a> {
    pub window: &'a Window,
    pub camera: &'a Camera,
    pub camera_transform: &'a GlobalTransform,
}

/// Transforms a point in screen space to a point in world space.
/// From https://bevy-cheatbook.github.io/cookbook/cursor2world.html
pub fn screen_to_world_pos(inputs: ScreenToWorldInputs, screen_pos: Vec2) -> Vec2 {
    let ScreenToWorldInputs {
        window,
        camera,
        camera_transform,
    } = inputs;

    let window_size = Vec2::new(window.width() as f32, window.height() as f32);
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    ndc_to_world.project_point3(ndc.extend(-1.0)).truncate()
}

#[derive(Debug, Clone, Copy)]
pub struct CursorToWorldInputs<'a> {
    pub windows: &'a Windows,
    pub camera: &'a Camera,
    pub camera_transform: &'a GlobalTransform,
}

/// A position on the grid calculated from a position in world space
#[derive(Debug, Clone, Copy)]
pub struct GridPos {
    /// The tile the position is over
    pub tile: IVec2,
    /// Where on the tile the position was over; 0 <= x <= 1
    pub hitvec: Vec2,
}

/// Converts a position on world space to a position in grid space
pub fn world_to_grid_pos(mut world_pos: Vec2) -> GridPos {
    world_pos += Vec2::splat(0.5);
    GridPos {
        tile: IVec2::new(world_pos.x.floor() as i32, world_pos.y.floor() as i32),
        hitvec: (world_pos.fract() + 1.0).fract(),
    }
}

pub fn screen_to_grid_pos(inputs: ScreenToWorldInputs, screen_pos: Vec2) -> GridPos {
    let world_pos = screen_to_world_pos(inputs, screen_pos);
    world_to_grid_pos(world_pos)
}

pub fn transform_from_grid_pos(grid_pos: IVec2, z: f32, facing_side: Side) -> Transform {
    Transform {
        translation: Vec3::new(grid_pos.x as f32, grid_pos.y as f32, z),
        rotation: facing_side.as_quat(),
        ..default()
    }
}
