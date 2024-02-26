pub mod paddles;
pub mod ball;
pub mod level;
pub mod scoreboard;
pub mod game_audio;
pub mod win_conditions;
pub mod camera;

use bevy::prelude::*;
use bevy_rapier2d::plugin::RapierPhysicsPlugin;

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
        .add_event::<GameFinishEvent>()
        .add_systems(OnEnter(AppState::Game), (
            setup_level,
            spawn_ball,
            load_audio_handlers,
            (setup_paddles_config, setup_paddles).chain(),
            setup_scoreboard
        ))
        .add_systems(OnExit(AppState::Game), (
            cleanup_audio_handlers,
            cleanup_paddles_config,
            reset_camera_position
        ))
        .add_systems(Update, (
            clamp_velocity,
            check_paddle_collision,
            check_ball_collision,
            check_goal_collision,
            move_paddle,
            update_scoreboard,
            check_score_win_condition,
            check_win_condition_events,
            update_camera_position
        ).run_if(in_state(GameState::UnPaused).and_then(in_state(AppState::Game))));
    }
}