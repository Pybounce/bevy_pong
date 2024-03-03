pub mod paddles;
pub mod ball;
pub mod level;
pub mod scoreboard;
pub mod game_audio;
pub mod win_conditions;
pub mod camera;
pub mod reset;

use bevy::prelude::*;

use self::ball::*;
use self::camera::*;
use self::game_audio::*;
use self::level::*;
use self::paddles::*;
use self::scoreboard::*;
use self::reset::*;
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
            (spawn_ball, reset_ball_data).chain(),
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
            update_camera_position,
            the_mayo_check
        ).run_if(in_state(GameState::Playing).and_then(in_state(AppState::Game))))
        .add_systems(OnEnter(GameState::Resetting), (setup_game_reset_data, start_resetting).chain())
        .add_systems(OnExit(GameState::Resetting), (finish_resetting, reset_ball_data))
        .add_systems(Update, (
            check_reset_state_end,
            //translation_lerp_score_reset
        ).run_if(in_state(GameState::Resetting).and_then(in_state(AppState::Game))));
    }
}

