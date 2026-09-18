use bevy::prelude::*;
use crate::{comps, input, enums::Direction, mapstate::MapState};


pub fn player_movement(
    impassable: Query<&comps::Impassable>,
    mut pos: Single<&mut comps::Pos, With<comps::Player>>,
    mut key: ResMut<input::InputBuffer>,
    mut map: ResMut<MapState>,
) {

    if !key.has_input() {
        return;
    }

    
    if key.consume_if(KeyCode::KeyW) || key.consume_if(KeyCode::ArrowUp) {
        move_entity(impassable, &mut map, &mut pos, Direction::Up);
    }
    else if key.consume_if(KeyCode::KeyS) || key.consume_if(KeyCode::ArrowDown) {
        move_entity(impassable, &mut map, &mut pos, Direction::Down);
    }
    else if key.consume_if(KeyCode::KeyA) || key.consume_if(KeyCode::ArrowLeft) {
        move_entity(impassable, &mut map, &mut pos, Direction::Left);
    }
    else if key.consume_if(KeyCode::KeyD) || key.consume_if(KeyCode::ArrowRight) {
        move_entity(impassable, &mut map, &mut pos, Direction::Right);
    }

}

pub fn move_entity(
    impassable: Query<&comps::Impassable>,
    map: &mut MapState, 
    pos: &mut comps::Pos, 
    direction: Direction
) -> bool {
    let next_tile = direction.offset_pos(pos.to_pos());
    let can_move = map[next_tile].iter().all(|e| impassable.get(*e).is_err());

    if can_move {
        pos.move_direction(direction);
    }
    can_move
}

fn register_positions(
    entites: Query<(Entity, &comps::Pos)>,
    mut world: ResMut<crate::mapstate::MapState>
) {

    for (entity, pos) in entites {
        world[pos.to_pos()].insert(entity);
    }
}

pub struct MovementSystem;
impl Plugin for MovementSystem {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, register_positions)
        .add_systems(Update, crate::mapstate::MapState::update)
        .add_systems(Update, crate::comps::Pos::update_last_pos);
    }
}