pub mod paddles;
pub mod ball;
pub mod level;
pub mod scoreboard;
pub mod game_audio;

use bevy::prelude::*;

use self::ball::*;
use self::game_audio::*;
use self::level::*;
use self::paddles::*;
use self::scoreboard::*;
use super::common::states::*;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::Game), (
            setup_level,
            spawn_ball,
            load_audio_handlers,
            (setup_paddles_config, setup_paddles).chain(),
            setup_scoreboard
        ))
        .add_systems(OnExit(AppState::Game), (
            cleanup_audio_handlers,
            cleanup_paddles_config
        ))
        .add_systems(Update, (
            clamp_velocity,
            check_paddle_collision,
            check_ball_collision,
            check_goal_collision,
            move_paddle,
            update_scoreboard
        ).run_if(in_state(GameState::UnPaused).and_then(in_state(AppState::Game))));
    }
}