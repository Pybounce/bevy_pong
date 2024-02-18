use bevy::{
    prelude::*, 
    window::close_on_esc,
};
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (spawn_paddles, spawn_camera))
    .add_systems(Update, close_on_esc)
    .run();
}

fn spawn_paddles(mut commands: Commands)
{
    commands.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::Rgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 },
                ..default()
            },
            ..default()
        });
}

fn spawn_camera(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());

}