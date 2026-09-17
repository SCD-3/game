use bevy::prelude::*;
use crate::GRID_SIZE;
use crate::mapstate::MapState;
use crate::enums::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Impassable;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Pos {
    x: i64,
    y: i64
}
impl Pos {

    pub fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    pub fn set(&mut self, x: i64, y: i64, entity: &Entity, map: &mut MapState) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = x;
        self.y = y;

        map[(old_x, old_y)].remove(entity);
        map[(self.x, self.y)].insert(*entity);
    }

    pub fn move_direction(&mut self, direction: Direction, entity: &Entity, map: &mut MapState) {
        match direction {
            Direction::Up        => self.set(self.x,       self.y + 1, entity, map),
            Direction::Down      => self.set(self.x,       self.y - 1, entity, map),
            Direction::Left      => self.set(self.x - 1,   self.y    , entity, map),
            Direction::Right     => self.set(self.x + 1,   self.y    , entity, map),
            Direction::UpLeft    => self.set(self.x - 1, self.y + 1, entity, map),
            Direction::UpRight   => self.set(self.x + 1, self.y + 1, entity, map),
            Direction::DownLeft  => self.set(self.x - 1, self.y - 1, entity, map),
            Direction::DownRight => self.set(self.x + 1, self.y - 1, entity, map),
        }
    }

    pub fn get_vec3_for_player_at(&self, player_pos: Pos) -> Vec3 {
        Vec3::new(
                (self.x - player_pos.x) as f32 * GRID_SIZE,
                (self.y - player_pos.y) as f32 * GRID_SIZE,
                0.0,
            )
    }

}