mod plugins;
use plugins::paddles::*;

use bevy::{
    prelude::*, 
    window::close_on_esc,
};


fn main() {
    App::new()
    .add_plugins((DefaultPlugins, PaddlesPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, close_on_esc)
    .run();
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}




