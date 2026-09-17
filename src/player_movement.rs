use bevy::prelude::*;
use crate::{comps, input, enums::Direction, mapstate::MapState};


pub fn player_movement(
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
        player.1.move_direction(Direction::Up, &entity, &mut map)
    }
    else if key.consume_if(KeyCode::KeyS) || key.consume_if(KeyCode::ArrowDown) {
        player.1.move_direction(Direction::Down, &entity, &mut map)
    }
    else if key.consume_if(KeyCode::KeyA) || key.consume_if(KeyCode::ArrowLeft) {
        player.1.move_direction(Direction::Left, &entity, &mut map)
    }
    else if key.consume_if(KeyCode::KeyD) || key.consume_if(KeyCode::ArrowRight) {
        player.1.move_direction(Direction::Right, &entity, &mut map)
    }

}