use bevy::{ecs::{event::{Event, EventReader, EventWriter}, schedule::NextState, system::{Res, ResMut}}, utils::info};
use super::{AppState, Scoreboard};


const SCORE_WIN_AMOUNT: usize = 5;


pub fn check_score_win_condition(scoreboard: Res<Scoreboard>, mut finish_event_writer: EventWriter<GameFinishEvent>) {
    if scoreboard.left_score >= SCORE_WIN_AMOUNT {
        finish_event_writer.send(GameFinishEvent::LeftWin);
    }
    else if scoreboard.right_score >= SCORE_WIN_AMOUNT {
        finish_event_writer.send(GameFinishEvent::RightWin);
    }
}


#[derive(Event)]
pub enum GameFinishEvent {
    LeftWin,
    RightWin,
    Draw
}

pub fn check_win_condition_events(
    mut finish_event_reader: EventReader<GameFinishEvent>,
    mut app_state: ResMut<NextState<AppState>>
) {
    for finish_event in finish_event_reader.read() {
        match finish_event {
            GameFinishEvent::LeftWin => info("LEFT WIN"),
            GameFinishEvent::RightWin => info("RIGHT WIN"),
            GameFinishEvent::Draw => info("DRAW")
        }
        app_state.set(AppState::MainMenu);
        break;
    }
}