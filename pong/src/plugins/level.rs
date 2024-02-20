
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::ball::{self, Ball};

const LEVEL_AREA: Vec2 = Vec2::new(1400.0, 700.0);

#[derive(Component)]
pub enum Goal {
    Left,
    Right
}


pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_level, setup_scoreboard));
        app.add_systems(Update, (check_goal_collisions, update_scoreboard));
        }
}


fn setup_level(mut commands: Commands) {

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, LEVEL_AREA.y / 2.0, 0.0),
            scale: Vec3::new(LEVEL_AREA.x, 1.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -(LEVEL_AREA.y / 2.0), 0.0),
            scale: Vec3::new(LEVEL_AREA.x, 1.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-(LEVEL_AREA.x / 2.0), 0.0, 0.0),
            scale: Vec3::new(1.0, LEVEL_AREA.y, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(Goal::Left)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(LEVEL_AREA.x / 2.0, 0.0, 0.0),
            scale: Vec3::new(1.0, LEVEL_AREA.y, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(Goal::Right)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0));

    
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec2::default().extend(-100.0),
            scale: Vec3::new(LEVEL_AREA.x, LEVEL_AREA.y, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgba(0.5, 0.5, 0.5, 1.0),
            ..default()
        },
        ..default()
    });


    }



fn check_goal_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<&mut Velocity, With<Ball>>,
    goal_query: Query<&Goal>,
    mut scoreboard: ResMut<Scoreboard>
) {
        for collision_event in collision_events.read() {

            let (entity1, entity2) = match collision_event {
                CollisionEvent::Started(e1, e2, _) => { (*e1, *e2) },
                CollisionEvent::Stopped(_, _, _) => { continue; },
            };
    
             let (ball_entity, goal_entity) = if ball_query.get(entity1).is_ok() {
                (entity1, entity2)
            } else if ball_query.get(entity2).is_ok() {
                (entity2, entity1)
            } else {
                continue;
            };
            let ball_velocity = ball_query.get(ball_entity).unwrap();
            if let Ok(goal) = goal_query.get(goal_entity) {
                match goal {
                    Goal::Left => {
                        scoreboard.right_score += 1;
                        info!("Left goal hit");

                    },
                    Goal::Right => {
                        scoreboard.left_score += 1;
                        info!("Right goal hit");
                    },
                }
            }
        }
    }



#[derive(Resource)]
struct Scoreboard {
    left_score: usize,
    right_score: usize
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
                color: Color::rgb(0.8, 0.8, 0.8),
                ..default()
            }),
            TextSection::new(
                "|",
                TextStyle {
                    font_size: 128.0,
                    color: Color::rgb(0.8, 0.8, 0.8),
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: 128.0,
                color: Color::rgb(0.8, 0.8, 0.8),
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        }),
    ));
}

fn update_scoreboard(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text, With<ScoreboardUI>>) {
    let mut text = query.single_mut();
    text.sections[0].value = scoreboard.left_score.to_string();
    text.sections[2].value = scoreboard.right_score.to_string();
}


