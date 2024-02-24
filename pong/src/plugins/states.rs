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
pub enum DespawnOnStateExit {
    App(AppState),
    Game(GameState)
}



pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
        .init_state::<GameState>()
        .add_systems(Update, switch_states)
        .add_systems(Update, check_exit_states);
    }
}






fn switch_states(input: Res<ButtonInput<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if input.just_pressed(KeyCode::KeyG) {
        app_state.set(AppState::Game);
    }
    if input.just_pressed(KeyCode::KeyM) {
        app_state.set(AppState::MainMenu);
    }
}

fn check_exit_states(mut game_state_transition_events: EventReader<StateTransitionEvent<GameState>>, mut app_state_transition_events: EventReader<StateTransitionEvent<AppState>>, mut commands: Commands, query: Query<(Entity, &DespawnOnStateExit), With<DespawnOnStateExit>>) {
    let game_state_events: Vec<_> = game_state_transition_events.read().collect();
    let app_state_events: Vec<_> = app_state_transition_events.read().collect();
    if app_state_events.len() == 0 && game_state_events.len() == 0 { return; }
    
    for (entity, state_lifetime) in query.iter() {
        match state_lifetime {
            DespawnOnStateExit::App(x) => {
                for ste in &app_state_events {
                    if x == &ste.before { commands.entity(entity).despawn(); } 
                }
            },
            DespawnOnStateExit::Game(x) => {
                for ste in &game_state_events {
                    if x == &ste.before { commands.entity(entity).despawn(); } 
                }
            },
        }
    }    
}