use bevy::prelude::*;
use crate::{comps, input, enums::Direction, mapstate::MapState};


pub fn player_movement(
    impassable: Query<&comps::Impassable>,
    mut player: Single<(Entity, &mut comps::Pos), With<comps::Player>>,
    mut key: ResMut<input::InputBuffer>,
    mut map: ResMut<MapState>,
) {

    if !key.has_input() {
        return;
    }

    let entity = player.0;
    // let mut pos = *player.1;

    
    if key.consume_if(KeyCode::KeyW) || key.consume_if(KeyCode::ArrowUp) {
        move_entity(impassable, entity, &mut map, &mut player.1, Direction::Up);
    }
    else if key.consume_if(KeyCode::KeyS) || key.consume_if(KeyCode::ArrowDown) {
        move_entity(impassable, entity, &mut map, &mut player.1, Direction::Down);
    }
    else if key.consume_if(KeyCode::KeyA) || key.consume_if(KeyCode::ArrowLeft) {
        move_entity(impassable, entity, &mut map, &mut player.1, Direction::Left);
    }
    else if key.consume_if(KeyCode::KeyD) || key.consume_if(KeyCode::ArrowRight) {
        move_entity(impassable, entity, &mut map, &mut player.1, Direction::Right);
    }

}

pub fn move_entity(
    impassable: Query<&comps::Impassable>,
    entity: Entity, 
    map: &mut MapState, 
    pos: &mut comps::Pos, 
    direction: Direction
) -> bool {
    let next_tile = direction.offset_pos(pos.to_pos());
    let can_move = map[next_tile].iter().all(|e| impassable.get(*e).is_err());

    if can_move {
        pos.move_direction(direction, entity, map);
    }
    can_move
}