pub mod paddles;
pub mod ball;
pub mod level;
pub mod scoreboard;
pub mod game_audio;
pub mod win_conditions;
pub mod camera;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::RigidBodyDisabled;
use bevy_rapier2d::dynamics::Sleeping;
use bevy_rapier2d::dynamics::Velocity;
use bevy_rapier2d::rapier::dynamics::RigidBody;
use bevy_rapier2d::rapier::dynamics::RigidBodyHandle;

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
        .add_systems(Update, (check_score_win_condition, check_win_condition_events).run_if(in_state(AppState::Game)))
        .add_systems(Update, (
            clamp_velocity,
            check_paddle_collision,
            check_ball_collision,
            check_goal_collision,
            move_paddles,
            update_scoreboard,
            update_camera_position
        ).run_if(in_state(GameState::Playing).and_then(in_state(AppState::Game))))
        .add_systems(OnEnter(GameState::Resetting), (setup_game_reset_data, disable_balls, disable_paddles))
        .add_systems(OnExit(GameState::Resetting), (enable_balls, enable_paddles))
        .add_systems(Update, (
            reset_camera_position,
            check_reset_state_end,
            reset_balls,
            reset_paddles
        ).run_if(in_state(GameState::Resetting).and_then(in_state(AppState::Game))));
    }
}

fn setup_game_reset_data(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(GameResetData { end_time: time.elapsed_seconds() + 1.0 })
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
    if dist < 0.01 { return; }
    let speed = dist / time_remaining.max(0.001);
    let direction = -camera_transform.translation.normalize();
    let movement = direction * speed * time.delta_seconds();
    camera_transform.translation += movement;
}
fn disable_balls(
    mut commands: Commands,
    mut ball_query: Query<Entity, With<Ball>>
) {
    for e in &mut ball_query {
        commands.entity(e).insert(RigidBodyDisabled);
    }
}

fn enable_balls(
    mut commands: Commands,
    mut ball_query: Query<(Entity, &mut Ball, &mut Velocity), With<Ball>>
) {
    for (entity, mut ball, mut velocity) in &mut ball_query {
        ball.current_max_velocity = ball.base_velocity;
        velocity.linvel = Vec2::new(ball.base_velocity, 0.0);
        commands.entity(entity).remove::<RigidBodyDisabled>();
    }
}

fn reset_balls(
    mut ball_query: Query<&mut Transform, With<Ball>>, 
    reset_data: Res<GameResetData>, 
    time: Res<Time>    
) {
    for mut t in &mut ball_query {
        let time_remaining = (reset_data.end_time - time.elapsed_seconds()).max(0.0);
        let dist = t.translation.length();
        if dist < 0.01 { continue; }
        let speed = dist / time_remaining.max(0.001);
        let direction = -t.translation.normalize();
        let movement = direction * speed * time.delta_seconds();
        t.translation += movement;
    }
}

fn disable_paddles(
    mut commands: Commands,
    mut query: Query<Entity, With<Paddle>>
) {
    for e in &mut query {
        commands.entity(e).insert(RigidBodyDisabled);
    }
}

fn enable_paddles(
    mut commands: Commands,
    mut query: Query<Entity, With<Paddle>>
) {
    for e in &mut query {

        commands.entity(e).remove::<RigidBodyDisabled>();
    }
}

fn reset_paddles(
    mut query: Query<&mut Transform, With<Paddle>>, 
    reset_data: Res<GameResetData>, 
    time: Res<Time>    
) {
    for mut t in &mut query {
        let time_remaining = (reset_data.end_time - time.elapsed_seconds()).max(0.0);
        let dist = t.translation.y.abs();
        if dist < 0.01 { continue; }
        let speed = dist / time_remaining.max(0.001);
        let direction = Vec3::new(0.0, -t.translation.y, 0.0).normalize();
        let movement = direction * speed * time.delta_seconds();
        t.translation += movement;
    }
}

fn check_reset_state_end(reset_data: Res<GameResetData>, time: Res<Time>, mut game_state: ResMut<NextState<GameState>>) {
    if reset_data.end_time < time.elapsed_seconds() {
        game_state.set(GameState::Playing);
    }
}

