mod game;
mod common;

use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy_rapier2d::prelude::*;

use bevy::{
    prelude::*, 
    window::close_on_esc,
};
use common::states::StatesPlugin;
use game::GamePlugin;


fn main() {
    let winit_settings = WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    };
    let window_settings = WindowPlugin {
        primary_window: Some(Window {
            title: "MOTHER HECKING PONG LETS GOOOOOOOOOOOOOOO!".into(),
            name: Some("pong app".into()),
            resolution: (1600., 900.).into(),
            ..default()
        }),
        ..default()
    };

    App::new()
    .insert_resource(winit_settings)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    //.add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins((DefaultPlugins.set(window_settings), StatesPlugin, GamePlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, close_on_esc)
    .run();
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


