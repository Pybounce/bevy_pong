use bevy::prelude::*;

use super::common::states::*;


pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), setup_menu)
        .add_systems(Update, (try_start_game).run_if(in_state(AppState::MainMenu)));
    }
}


fn setup_menu(mut commands: Commands) {
    commands.spawn(TextBundle {
        text: Text::from_section(
               "Press any key to play!",
               TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        style: Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::MainMenu));
}

fn try_start_game(input: Res<ButtonInput<KeyCode>>, mut app_state: ResMut<NextState<AppState>>) {
    if input.get_just_released().len() > 0 {
        app_state.set(AppState::Game);
    }
}