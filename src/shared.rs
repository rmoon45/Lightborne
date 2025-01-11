use bevy_rapier2d::prelude::*;

pub struct GroupLabel;

impl GroupLabel {
    pub const PLAYER_COLLIDER: Group = Group::GROUP_1;
    pub const PLAYER_SENSOR: Group = Group::GROUP_2;
    pub const TERRAIN: Group = Group::GROUP_3;
    pub const LIGHT_RAY: Group = Group::GROUP_4;
    pub const LIGHT_SENSOR: Group = Group::GROUP_5;
    pub const HURT_BOX: Group = Group::GROUP_6;
    pub const WHITE_RAY: Group = Group::GROUP_7;
}
