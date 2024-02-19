mod plugins;
mod components;
mod systems;
use bevy_rapier2d::prelude::*;
use plugins::paddles::*;
use plugins::ball::*;

use bevy::{
    prelude::*, 
    window::close_on_esc,
};


fn main() {
    App::new()
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins((DefaultPlugins, PaddlesPlugin, BallPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, close_on_esc)
    .run();
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}




