mod plugins;
mod states;
use bevy::winit::UpdateMode;
use bevy::winit::WinitSettings;
use bevy_rapier2d::prelude::*;
use plugins::level::LevelPlugin;
use plugins::paddles::PaddlesPlugin;
use plugins::ball::BallPlugin;
use plugins::scoreboard::ScoreboardPlugin;
use plugins::game_audio::GameAudioPlugin;
use states::*;

use bevy::{
    prelude::*, 
    window::close_on_esc,
};


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
    .init_state::<AppState>()
    .init_state::<GameState>()
    .insert_resource(winit_settings)
    .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    //.add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins((DefaultPlugins.set(window_settings), PaddlesPlugin, BallPlugin, LevelPlugin, ScoreboardPlugin, GameAudioPlugin))
    .add_systems(Startup, spawn_camera)
    .add_systems(Update, close_on_esc)
    .add_systems(Update, switch_states)
    .add_systems(OnExit(AppState::Game), exit_game_app_state_lifetime)
    .run();
}



fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}


fn switch_states(input: Res<ButtonInput<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::KeyG) {
        app_state.set(AppState::Game);
    }
    if input.just_pressed(KeyCode::KeyM) {
        app_state.set(AppState::MainMenu);
    }
}

fn exit_game_app_state_lifetime(mut commands: Commands, query: Query<(Entity, &AppStateLifetime), With<AppStateLifetime>>) {
    for (entity, state_lifetime) in query.iter() {
        match state_lifetime {
            AppStateLifetime::Game => commands.entity(entity).despawn(),
            _ => continue
        }
    }
}

