
use bevy::prelude::*;
use super::{super::common::states::*, LEVEL_AREA};


#[derive(Resource)]
pub struct Scoreboard {
    pub left_score: usize,
    pub right_score: usize
}

#[derive(Component)]
pub struct ScoreboardUI;

pub fn setup_scoreboard(mut commands: Commands) {

    commands.insert_resource(Scoreboard { left_score: 0, right_score: 0 });

    commands.spawn((
        ScoreboardUI,
        Text2dBundle {
        text: Text::from_sections([
            TextSection::from_style(TextStyle {
                font_size: 128.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            }),
            TextSection::new(
                "         ",
                TextStyle {
                    font_size: 128.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..Default::default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 128.0,
                color: Color::rgb(1.0, 1.0, 1.0),
                ..Default::default()
            }),
        ]),
        transform: Transform::from_xyz(0.0, LEVEL_AREA.y / 3.0, 0.0),
        ..default()
        }
    ))
    .insert(DespawnOnStateExit::App(AppState::Game));

}

pub fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUI>>) {
    let mut text = query.single_mut();

    text.sections[0].value = format!("{:02}", scoreboard.left_score).to_string();
    text.sections[2].value = format!("{:02}", scoreboard.right_score).to_string();
}