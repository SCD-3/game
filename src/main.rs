mod comps;
mod input;
use bevy::prelude::*;

const GRID_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(5.0))
        .add_systems(Startup, setup)
        .add_systems(Update, render)

        // input system
        .insert_resource(input::InputQueue::default())
        .add_systems(Update, input::input_system)

        .add_systems(FixedUpdate, player_movement)

        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        comps::Player, 
        comps::Pos { x: 0, y: 0 },
        Sprite::from_color(
            Color::srgb(1.0, 0.0, 0.0), 
            Vec2::new(GRID_SIZE, GRID_SIZE)
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}



fn player_movement(
    mut player: Single<&mut comps::Pos, With<comps::Player>>,
    mut key: ResMut<input::InputQueue>
) {

    let Some(input) = key.queue.front() else {return;};
    match input {
            KeyCode::KeyW | KeyCode::ArrowUp => player.y += 1,
            KeyCode::KeyS | KeyCode::ArrowDown => player.y -= 1,
            KeyCode::KeyA | KeyCode::ArrowLeft => player.x -= 1,
            KeyCode::KeyD | KeyCode::ArrowRight => player.x += 1,
            _ => return,
        }
    
    key.queue.pop_front();

}

fn render(
    mut query: Query<(&comps::Pos, &mut Transform)>
) {

    for (pos, mut transform) in &mut query {
            transform.translation = Vec3::new(
                pos.x as f32 * GRID_SIZE,
                pos.y as f32 * GRID_SIZE,
                0.0,
            );
        }
}