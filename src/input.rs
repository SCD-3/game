use bevy::{platform::collections::HashMap, prelude::*};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputAction {
    PassTurn,
    PlayerMoveUp,
    PlayerMoveDown,
    PlayerMoveLeft,
    PlayerMoveRight
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputBind {
    pub primary: Option<KeyCode>,
    pub secondary: Option<KeyCode>
}
impl InputBind {
    
    pub fn new(primary: Option<KeyCode>, secondary: Option<KeyCode>) -> Self {
        InputBind { primary, secondary }
    }

}

#[derive(Resource)]
pub struct InputMapping {
    pub mapping: HashMap<InputAction, InputBind>
}
impl InputMapping {

    pub fn action_taken(&self, action: InputAction, input_buffer: &mut InputBuffer) -> bool {
        let InputBind { primary, secondary } = self.mapping.get(&action)
            .unwrap_or_else(|| panic!("failed to find bind for {action:?}"));
        
          primary.is_some_and(|k| input_buffer.consume_if(k)) || 
        secondary.is_some_and(|k| input_buffer.consume_if(k))
    }

}

impl Default for InputMapping {
    
    fn default() -> Self {
        let mut mapping = HashMap::new();
        mapping.insert(InputAction::PassTurn       , InputBind::new(Some(KeyCode::Space), None                     ));
        mapping.insert(InputAction::PlayerMoveUp   , InputBind::new(Some(KeyCode::KeyW ), Some(KeyCode::ArrowUp   )));
        mapping.insert(InputAction::PlayerMoveDown , InputBind::new(Some(KeyCode::KeyS ), Some(KeyCode::ArrowDown )));
        mapping.insert(InputAction::PlayerMoveLeft , InputBind::new(Some(KeyCode::KeyA ), Some(KeyCode::ArrowLeft )));
        mapping.insert(InputAction::PlayerMoveRight, InputBind::new(Some(KeyCode::KeyD ), Some(KeyCode::ArrowRight)));

        InputMapping { mapping }
    }

}


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
        .insert_resource(InputMapping::default())
        .insert_resource(InputBuffer::default())
        .add_systems(Update, buffer_input);
    }
}