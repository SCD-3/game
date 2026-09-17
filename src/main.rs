mod comps;
mod input;
use bevy::prelude::*;

const GRID_SIZE: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::InputSystem)

        .insert_resource(Time::<Fixed>::from_hz(10.0))
        .add_systems(Startup, setup)
        .add_systems(Update, render)


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