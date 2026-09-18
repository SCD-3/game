use bevy::ecs::lifecycle::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use crate::GRID_SIZE;
use crate::mapstate::MapState;
use crate::enums::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Impassable;

#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[require(LastPos)]
#[component(on_insert = Pos::on_insert)]
pub struct Pos {
    x: i64,
    y: i64
}
impl Pos {

    pub fn new(x: i64, y: i64) -> Self {
        Pos { x, y }
    }

    pub fn set(&mut self, x: i64, y: i64) {
        self.x = x;
        self.y = y;
    }

    pub fn to_pos(&self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up        => self.set(self.x,       self.y + 1),
            Direction::Down      => self.set(self.x,       self.y - 1),
            Direction::Left      => self.set(self.x - 1,   self.y    ),
            Direction::Right     => self.set(self.x + 1,   self.y    ),
            Direction::UpLeft    => self.set(self.x - 1, self.y + 1),
            Direction::UpRight   => self.set(self.x + 1, self.y + 1),
            Direction::DownLeft  => self.set(self.x - 1, self.y - 1),
            Direction::DownRight => self.set(self.x + 1, self.y - 1),
        }
    }

    pub fn get_vec3_for_player_at(&self, player_pos: Pos) -> Vec3 {
        Vec3::new(
                (self.x - player_pos.x) as f32 * GRID_SIZE,
                (self.y - player_pos.y) as f32 * GRID_SIZE,
                0.0,
            )
    }

    pub fn update_last_pos(
        mut query: Query<(&Self, &mut LastPos), Changed<Self>>
    ) {
        for (pos, mut last) in &mut query {
            last.update(*pos);
        }
    }

    fn on_insert(
    mut world: DeferredWorld,
    // entity: Entity,
    hook: HookContext,
    ) {
        let entity = hook.entity;

        let pos = *world.get::<Pos>(entity).unwrap();
        let mut grid = world.get_resource_mut::<MapState>().unwrap();

        grid[pos.to_pos()].insert(entity);
    }

}

#[derive(Component, Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LastPos {
    prev: Option<Pos>
}
impl LastPos {
    pub fn update(&mut self, pos: Pos) {
        self.prev = Some(pos)
    }

    pub fn clear(&mut self) {
        self.prev = None
    }

    pub fn prev(&self) -> Option<Pos> {
        self.prev
    }
}