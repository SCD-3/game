mod comps;
mod input;
mod player_movement;

use bevy::prelude::*;

const GRID_SIZE: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::InputSystem)

        .insert_resource(Time::<Fixed>::from_hz(10.0))
        .add_systems(Startup, setup)
        .add_systems(Update, render)
        
        
        .add_systems(FixedUpdate, player_movement::player_movement)

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

    commands.spawn((
        comps::Pos { x: 1, y: 0 },
        comps::Impassable,
        Sprite::from_color(
            Color::srgb(0.6, 0.6, 0.6), 
            Vec2::new(GRID_SIZE, GRID_SIZE)
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    commands.spawn((
        comps::Pos { x: 1, y: -1 },
        comps::Impassable,
        Sprite::from_color(
            Color::srgb(0.6, 0.6, 0.6),
            Vec2::new(GRID_SIZE, GRID_SIZE)
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    commands.spawn((
        comps::Pos { x: 1, y: -2 },
        comps::Impassable,
        Sprite::from_color(
            Color::srgb(0.6, 0.6, 0.6),
            Vec2::new(GRID_SIZE, GRID_SIZE)
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    commands.spawn((
        comps::Pos { x: 0, y: -2 },
        comps::Impassable,
        Sprite::from_color(
            Color::srgb(0.6, 0.6, 0.6),
            Vec2::new(GRID_SIZE, GRID_SIZE)
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));

    commands.spawn((
        comps::Pos { x: -1, y: -2 },
        comps::Impassable,
        Sprite::from_color(
            Color::srgb(0.6, 0.6, 0.6),
            Vec2::new(GRID_SIZE, GRID_SIZE)
        ),
        Transform::from_xyz(0.0, 0.0, 0.0)
    ));
}

fn render(
    mut query: Query<(&comps::Pos, &mut Transform), Without<comps::Player>>,
    player: Single<(&comps::Pos, &mut Transform), With<comps::Player>>
) {

    // player.1.translation = Vec3::new(0.0, 0.0, 0.0);
    // player relative pos is constant

    for (pos, mut transform) in &mut query {
            transform.translation = pos.get_vec3_for_player_at(*player.0);
        }
}