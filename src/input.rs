use bevy::prelude::*;
use std::collections::VecDeque;

const MAX_INPUT_LENGHT: usize = 5;

#[derive(Resource, Default)]
pub struct InputQueue {
    pub queue: VecDeque<KeyCode>,
}

pub fn input_system(
    mut input_queue: ResMut<InputQueue>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard.get_just_pressed() {
        input_queue.queue.push_back(*key);
        
        if input_queue.queue.len() > MAX_INPUT_LENGHT {
            input_queue.queue.pop_front();
        }
    }
}