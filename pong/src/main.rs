mod game;
mod common;
mod main_menu;

use bevy::core_pipeline::bloom::BloomSettings;
use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy_rapier2d::prelude::*;
use bevy_kira_audio::AudioPlugin as KiraAudioPlugin;

use bevy::{
    prelude::*, 
    window::close_on_esc,
};
use common::states::StatesPlugin;
use common::tweening::TweenPlugin;
use game::reset::ScoreTranslationLerpReset;
use game::GamePlugin;
use main_menu::MainMenuPlugin;

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
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    //.add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins((DefaultPlugins.set(window_settings), KiraAudioPlugin, StatesPlugin, MainMenuPlugin, GamePlugin, TweenPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, close_on_esc)
    .run();
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(BloomSettings::default())
        .insert(ScoreTranslationLerpReset { reset_translation: Vec3::default() });
}


