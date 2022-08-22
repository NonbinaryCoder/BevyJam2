use bevy::prelude::*;
use std::{f32::consts::PI, ops::*};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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
    pub fn as_angle(self) -> f32 {
        use Side::*;
        match self {
            North => 0.0,
            East => PI / 2.0,
            South => PI,
            West => (PI * 3.0) / 2.0,
        }
    }

    /// Returns a quat facing towards this side, with `north` being the identity
    pub fn as_quat(self) -> Quat {
        use Side::*;
        match self {
            North => Quat::IDENTITY,
            East => Quat::from_rotation_z(PI / 2.0),
            South => Quat::from_rotation_z(PI),
            West => Quat::from_rotation_z((PI * 3.0) / 2.0),
        }
    }

    /// Figures out which side a hitvec is closest to
    pub fn from_hitvec(hitvec: Vec2) -> Side {
        let hitvec = hitvec - Vec2::splat(0.5);
        SideArr {
            north: hitvec.y,
            east: hitvec.x,
            south: -hitvec.y,
            west: -hitvec.x,
        }
        .into_iter_labeled()
        .max_by(|(_, x), (_, y)| x.total_cmp(y))
        .unwrap()
        .0
    }
}

#[derive(Debug)]
pub struct SideArr<T> {
    pub north: T,
    pub east: T,
    pub south: T,
    pub west: T,
}

impl<T> SideArr<T> {
    /// Consumes this,
    /// returning an iterator over it's elements in NESW order,
    /// labelled with the side they are from
    pub fn into_iter_labeled(self) -> std::array::IntoIter<(Side, T), 4> {
        use Side::*;
        [
            (North, self.north),
            (East, self.east),
            (South, self.south),
            (West, self.west),
        ]
        .into_iter()
    }
}

impl<T> Index<Side> for SideArr<T> {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        use Side::*;
        match index {
            North => &self.north,
            East => &self.east,
            South => &self.south,
            West => &self.west,
        }
    }
}
