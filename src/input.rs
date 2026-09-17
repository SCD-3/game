use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct InputBuffer {
    buffer: Option<KeyCode>,
}
impl InputBuffer {

    pub fn check(&self) -> Option<KeyCode> {
        self.buffer
    }

    pub fn consume(&mut self) -> bool {
        let b = self.buffer;
        self.buffer = None;
        b.is_some()
    }

    pub fn consume_if(&mut self, target: KeyCode) -> bool {
        if self.check() == Some(target) {
            self.consume();
            true
        }
        else {
            false
        }
    }

    pub fn has_input(&self) -> bool {
        self.buffer.is_some()
    }

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