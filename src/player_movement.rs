use bevy::prelude::*;
use crate::{comps, input};


pub fn player_movement(
    mut player: Single<&mut comps::Pos, With<comps::Player>>,
    mut key: ResMut<input::InputBuffer>
) {

    if !key.has_input() {
        return;
    }
    
    if key.consume_if(KeyCode::KeyW) || key.consume_if(KeyCode::ArrowUp) {
        player.y += 1;
    }
    else if key.consume_if(KeyCode::KeyS) || key.consume_if(KeyCode::ArrowDown) {
        player.y -= 1;
    }
    else if key.consume_if(KeyCode::KeyA) || key.consume_if(KeyCode::ArrowLeft) {
        player.x -= 1;
    }
    else if key.consume_if(KeyCode::KeyD) || key.consume_if(KeyCode::ArrowRight) {
        player.x += 1;
    }

}