use bevy::prelude::*;

use super::common::states::*;


pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), (empty))
        .add_systems(OnExit(AppState::MainMenu), (empty))
        .add_systems(Update, (empty).run_if(in_state(AppState::MainMenu)));
    }
}


fn setup_menu() {

}

fn empty() {

}