use bevy::prelude::*;
use crate::{comps, input, enums::Direction, mapstate::MapState, ticks};


pub fn register_player_move_input(
    impassable_map: Query<&comps::Impassable>,
    mut pos: Single<&mut comps::Pos, With<comps::Player>>,
    mut key_buffer: ResMut<input::InputBuffer>,
        key: Res<input::InputMapping>,
    mut map: ResMut<MapState>,
    mut tick_request: MessageWriter<ticks::RequestNextTick>
) {

    
    if key.action_taken(input::InputAction::PlayerMoveUp, &mut key_buffer) {
        move_entity(impassable_map, &mut map, &mut pos, Direction::Up);
    }
    else if key.action_taken(input::InputAction::PlayerMoveDown, &mut key_buffer) {
        move_entity(impassable_map, &mut map, &mut pos, Direction::Down);
    }
    else if key.action_taken(input::InputAction::PlayerMoveLeft, &mut key_buffer) {
        move_entity(impassable_map, &mut map, &mut pos, Direction::Left);
    }
    else if key.action_taken(input::InputAction::PlayerMoveRight, &mut key_buffer) {
        move_entity(impassable_map, &mut map, &mut pos, Direction::Right);
    }

    else {
        return;
    }

    // run tick only if player did acually move
    tick_request.write(ticks::RequestNextTick);

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
        // .add_systems(ticks::PreTick, register_positions)
        .add_systems(ticks::PreTick, crate::mapstate::MapState::update)
        .add_systems(ticks::PostTick, crate::comps::Pos::update_last_pos)
        .add_systems(input::InputStep, register_player_move_input);
    }
}