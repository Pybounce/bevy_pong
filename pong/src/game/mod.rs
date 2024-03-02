pub mod paddles;
pub mod ball;
pub mod level;
pub mod scoreboard;
pub mod game_audio;
pub mod win_conditions;
pub mod camera;
pub mod shared;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::dynamics::RigidBodyDisabled;
use bevy_rapier2d::dynamics::Velocity;
use rand::*;

use self::ball::*;
use self::camera::*;
use self::game_audio::*;
use self::level::*;
use self::paddles::*;
use self::scoreboard::*;
use self::shared::ScoreTranslationLerpReset;
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
        .add_systems(OnExit(AppState::Game), cleanup_audio_handlers)
        .add_systems(Update, (check_score_win_condition, check_win_condition_events, update_scoreboard).run_if(in_state(AppState::Game)))
        .add_systems(Update, (
            clamp_velocity,
            check_paddle_collision,
            check_ball_collision,
            check_goal_collision,
            move_paddles,
            update_camera_position
        ).run_if(in_state(GameState::Playing).and_then(in_state(AppState::Game))))
        .add_systems(OnEnter(GameState::Resetting), (setup_game_reset_data, disable_rigidbodies_on_reset_entities))
        .add_systems(OnExit(GameState::Resetting), (enable_rigidbodies_on_reset_entities, reset_ball_data))
        .add_systems(Update, (
            check_reset_state_end,
            translation_lerp_score_reset
        ).run_if(in_state(GameState::Resetting).and_then(in_state(AppState::Game))));
    }
}

fn setup_game_reset_data(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(GameResetData { end_time: time.elapsed_seconds() + 1.0 })
}

fn check_reset_state_end(reset_data: Res<GameResetData>, time: Res<Time>, mut game_state: ResMut<NextState<GameState>>) {
    if reset_data.end_time < time.elapsed_seconds() {
        game_state.set(GameState::Playing);
    }
}

///so I can't name stuff well, fuck off
fn disable_rigidbodies_on_reset_entities(
    mut commands: Commands,
    mut query: Query<Entity, (With<ScoreTranslationLerpReset>, With<RigidBody>)>
) {
    for e in &mut query {
        commands.entity(e).insert(RigidBodyDisabled);
    }
}
fn enable_rigidbodies_on_reset_entities(
    mut commands: Commands,
    mut query: Query<Entity, (With<ScoreTranslationLerpReset>, With<RigidBody>)>
) {
    for e in &mut query {
        commands.entity(e).remove::<RigidBodyDisabled>();
    }
}
fn reset_ball_data(
    mut query: Query<(&mut Ball, &mut Velocity)>
) {
    for (mut ball, mut velocity) in &mut query {
        ball.current_max_velocity = ball.base_velocity;
        let y: f32 = thread_rng().gen::<f32>() - 0.5;
        let x: f32 = 1.0 - y.abs(); //always positive x towards enemy
        velocity.linvel = Vec2::new(x, y) * ball.current_max_velocity;
    }
}
fn translation_lerp_score_reset(
    mut query: Query<(&mut Transform, &ScoreTranslationLerpReset)>,
    reset_data: Res<GameResetData>, 
    time: Res<Time> 
) {
    const OK_DISTANCE: f32 = 0.01;
    let time_remaining = reset_data.end_time - time.elapsed_seconds();
    if time_remaining <= 0.0 { return; }

    for (mut transform, lerp_reset_data) in &mut query {
        let delta = lerp_reset_data.reset_translation - transform.translation;
        let dist = delta.length();
        if dist < OK_DISTANCE { continue; }
        let speed = dist / time_remaining.max(0.001);
        let mut step = delta.normalize() * speed * time.delta_seconds();
        if step.length() > dist { step = delta; }
        transform.translation += step;
    }
}
