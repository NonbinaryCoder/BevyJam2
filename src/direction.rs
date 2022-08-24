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

macro_rules! convert_to_2d_vector {
    ($name:ident, $type:ident) => {
        pub fn $name(self) -> $type {
            use Side::*;
            match self {
                North => $type::Y,
                East => $type::X,
                South => -$type::Y,
                West => -$type::X,
            }
        }
    };
}

macro_rules! rotate_2d_vector {
    ($name:ident, $type:ident) => {
        /// Takes a vector oriented `North` and returns one oriented this direction
        #[must_use]
        pub fn $name(self, v: $type) -> $type {
            use Side::*;
            match self {
                North => v,
                East => $type::new(v.y, -v.x),
                South => $type::new(-v.x, -v.y),
                West => $type::new(-v.y, v.x),
            }
        }
    };
}

impl Side {
    #[must_use]
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

    convert_to_2d_vector!(to_vec2, Vec2);
    convert_to_2d_vector!(to_ivec2, IVec2);

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

    rotate_2d_vector!(rotate_vec2, Vec2);
}

#[derive(Debug)]
pub struct SideArr<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
}
