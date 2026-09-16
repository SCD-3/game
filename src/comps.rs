use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Pos {
    pub x: i64,
    pub y: i64
}