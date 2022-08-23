use bevy::prelude::*;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis2d {
    X,
    Y,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    #[default]
    North,
    East,
    South,
    West,
}

impl Side {
    pub fn opposite(self) -> Side {
        use Side::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }

    pub fn rotate_left(self) -> Side {
        use Side::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }

    pub fn rotate_right(self) -> Side {
        use Side::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    /// Returns this as an angle in radians, with `North` being 0
    pub fn to_angle(self) -> f32 {
        use Side::*;
        match self {
            North => 0.0,
            East => (PI * 3.0) / 2.0,
            South => PI,
            West => PI / 2.0,
        }
    }

    pub fn to_vec2(self) -> Vec2 {
        use Side::*;
        match self {
            North => Vec2::Y,
            East => Vec2::X,
            South => -Vec2::Y,
            West => -Vec2::X,
        }
    }

    /// Returns a quat facing towards this side, with `North` being the identity
    pub fn to_quat(self) -> Quat {
        Quat::from_rotation_z(self.to_angle())
    }

    /// Returns the axis this faces (for example `North` is `Y`)
    pub fn axis(self) -> Axis2d {
        use Side::*;
        match self {
            North | South => Axis2d::Y,
            East | West => Axis2d::X,
        }
    }
}

#[derive(Debug)]
pub struct SideArr<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
}
