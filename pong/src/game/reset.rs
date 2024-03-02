use bevy::prelude::*;
use rand::*;
use bevy_rapier2d::dynamics::RigidBody;
use bevy_rapier2d::dynamics::RigidBodyDisabled;
use bevy_rapier2d::dynamics::Velocity;

use crate::common::tweening::PositionTween;

use super::Ball;
use super::GameResetData;
use super::GameState;

#[derive(Component)]
pub struct ScoreTranslationLerpReset {
    pub reset_translation: Vec3
}



pub fn setup_game_reset_data(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(GameResetData { end_time: time.elapsed_seconds() + 1.0 })
}

pub fn check_reset_state_end(reset_data: Res<GameResetData>, time: Res<Time>, mut game_state: ResMut<NextState<GameState>>) {
    if reset_data.end_time < time.elapsed_seconds() {
        game_state.set(GameState::Playing);
    }
}

///so I can't name stuff well, fuck off
pub fn start_resetting(
    mut commands: Commands,
    mut query: Query<(Entity, &ScoreTranslationLerpReset, &Transform), With<RigidBody>>,
    reset_data: Res<GameResetData>,
    time: Res<Time> 
) {
    for (e, lerp_reset_data, transform) in &mut query {
        let reset_position_offset = lerp_reset_data.reset_translation - transform.translation;
        commands.entity(e)
        .insert(RigidBodyDisabled)
        .insert(PositionTween::new(time.elapsed_seconds(), reset_data.end_time - time.elapsed_seconds(), reset_position_offset));
    }
}
pub fn finish_resetting(
    mut commands: Commands,
    mut query: Query<Entity, (With<ScoreTranslationLerpReset>, With<RigidBody>)>
) {
    for e in &mut query {
        commands.entity(e).remove::<RigidBodyDisabled>();
    }
}
pub fn reset_ball_data(
    mut query: Query<(&mut Ball, &mut Velocity)>
) {
    for (mut ball, mut velocity) in &mut query {
        ball.current_max_velocity = ball.base_velocity;
        let y: f32 = thread_rng().gen::<f32>() - 0.5;
        let x: f32 = 1.0 - y.abs(); //always positive x towards enemy
        velocity.linvel = Vec2::new(x, y) * ball.current_max_velocity;
    }
}
pub fn translation_lerp_score_reset(
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
