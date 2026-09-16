use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub buffer: Option<KeyCode>,
}

fn buffer_input(
    mut input_queue: ResMut<InputBuffer>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for key in keyboard.get_just_pressed() {
        input_queue.buffer = Some(*key)
    }
}


pub struct InputSystem;
impl Plugin for InputSystem {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(InputBuffer::default())
        .add_systems(Update, buffer_input);
    }
}