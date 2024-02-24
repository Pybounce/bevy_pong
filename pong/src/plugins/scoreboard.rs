
use bevy::prelude::*;

use crate::DespawnOnStateExit;

use super::super::{AppState, GameState};

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_scoreboard);
        app.add_systems(Update, update_scoreboard.run_if(in_state(GameState::UnPaused).and_then(in_state(AppState::Game))));
        }
}

#[derive(Resource)]
pub struct Scoreboard {
    pub left_score: usize,
    pub right_score: usize
}

#[derive(Component)]
struct ScoreboardUI;

fn setup_scoreboard(mut commands: Commands) {

    commands.insert_resource(Scoreboard { left_score: 0, right_score: 0 });

    commands.spawn((
        ScoreboardUI,
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 128.0,
                color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
            TextSection::new(
                "         ",
                TextStyle {
                    font_size: 128.0,
                    color: Color::rgb(0.5, 0.5, 0.5),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 128.0,
                color: Color::rgb(0.5, 0.5, 0.5),
                ..default()
            }),
        ])
        .with_style(Style {
            width: Val::Auto,
            position_type: PositionType::Absolute,
            margin: UiRect {bottom: Val::Px(400.0), ..default()},
            display: Display::Flex,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        }),
    ))
    .insert(DespawnOnStateExit(AppState::Game));
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUI>>) {
    let mut text = query.single_mut();

    text.sections[0].value = format!("{:02}", scoreboard.left_score).to_string();
    text.sections[2].value = format!("{:02}", scoreboard.right_score).to_string();
}