pub mod paddles;
pub mod ball;
pub mod level;
pub mod scoreboard;
pub mod game_audio;
pub mod win_conditions;
pub mod camera;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use self::ball::*;
use self::camera::*;
use self::game_audio::*;
use self::level::*;
use self::paddles::*;
use self::scoreboard::*;
use self::win_conditions::*;
use self::win_conditions::GameFinishEvent;
use super::common::states::*;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<GameResetData>()
        .add_event::<GameFinishEvent>()
        .add_systems(OnEnter(AppState::Game), (
            setup_level,
            spawn_ball,
            load_audio_handlers,
            setup_paddles,
            setup_scoreboard
        ))
        .add_systems(OnExit(AppState::Game), (
            cleanup_audio_handlers,
            reset_camera_position
        ))
        .add_systems(Update, (
            clamp_velocity,
            check_paddle_collision,
            check_ball_collision,
            check_goal_collision,
            move_paddles,
            update_scoreboard,
            check_score_win_condition,
            check_win_condition_events,
            update_camera_position
        ).run_if(in_state(GameState::Playing).and_then(in_state(AppState::Game))))
        .add_systems(OnEnter(GameState::Resetting), setup_game_reset_data)
        .add_systems(Update, (
            reset_camera_position,
            check_reset_state_end,
            reset_ball
        ).run_if(in_state(GameState::Resetting).and_then(in_state(AppState::Game))));
    }
}

fn setup_game_reset_data(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(GameResetData { end_time: time.elapsed_seconds() + 2.0 })
}

fn reset_camera_position(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>, 
    reset_data: Res<GameResetData>, 
    time: Res<Time>
) {
    let mut camera_transform = match camera_transform_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return
    };

    let time_remaining = (reset_data.end_time - time.elapsed_seconds()).max(0.0);
    let dist = camera_transform.translation.length();
    let speed = dist / time_remaining.max(0.001);
    let direction = -camera_transform.translation.normalize();
    let movement = direction * speed * time.delta_seconds();
    camera_transform.translation += movement;
    warn!("c movement {}", movement);
    warn!("c time_remaining {}", time_remaining);
    warn!("c direction {}", direction)
}

fn reset_ball(
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>, 
    reset_data: Res<GameResetData>, 
    time: Res<Time>    
) {
    for (mut t, mut v) in &mut ball_query {
        let time_remaining = (reset_data.end_time - time.elapsed_seconds()).max(0.0);
        let dist = t.translation.length();
        let speed = dist / time_remaining.max(0.001);
        let direction = -t.translation.normalize();
        let movement = direction * speed * time.delta_seconds();
        t.translation += movement;
        v.linvel = Vec2::new(0.001, 0.0);
        warn!("movement {}", movement);
        warn!("time_remaining {}", time_remaining);
        warn!("direction {}", direction);
    }
}

fn check_reset_state_end(reset_data: Res<GameResetData>, time: Res<Time>, mut game_state: ResMut<NextState<GameState>>) {
    if reset_data.end_time < time.elapsed_seconds() {
        game_state.set(GameState::Playing);
    }
}

