use bevy::prelude::*;

use bevy::prelude::States;

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum AppState {
    #[default]
    Game,
    MainMenu
}

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    Paused,
    #[default]
    UnPaused
}

//entities with state lifetime x, will be removed when state x is exited
#[derive(Component)]
pub struct DespawnOnStateExit(pub AppState);
