use bevy::prelude::*;
use crate::GRID_SIZE;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Impassable;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i64,
    pub y: i64
}
impl Pos {

    pub fn get_vec3_for_player_at(&self, player_pos: Pos) -> Vec3 {
        Vec3::new(
                (self.x - player_pos.x) as f32 * GRID_SIZE,
                (self.y - player_pos.y) as f32 * GRID_SIZE,
                0.0,
            )
    }

}