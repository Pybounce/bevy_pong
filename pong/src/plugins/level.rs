
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::super::*;
use super::ball::Ball;
use super::scoreboard::Scoreboard;

const LEVEL_AREA: Vec2 = Vec2::new(1400.0, 700.0);

#[derive(Component)]
pub enum Goal {
    Left,
    Right
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), setup_level);
        app.add_systems(Update, check_goal_collisions.run_if(in_state(GameState::UnPaused).and_then(in_state(AppState::Game))));
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
    .insert(Friction::coefficient(0.0))
    .insert(AppStateLifetime::Game);

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
    .insert(Friction::coefficient(0.0))
    .insert(AppStateLifetime::Game);

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
    .insert(Friction::coefficient(0.0))
    .insert(AppStateLifetime::Game);

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
    .insert(Friction::coefficient(0.0))
    .insert(AppStateLifetime::Game);

    
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec2::default().extend(-100.0),
            scale: Vec3::new(LEVEL_AREA.x, LEVEL_AREA.y, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgba(0.15, 0.15, 0.15, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(AppStateLifetime::Game);


    }

fn check_goal_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<(), With<Ball>>,
    goal_query: Query<&Goal>,
    mut scoreboard: ResMut<Scoreboard>
) {
        for collision_event in collision_events.read() {

            let (entity1, entity2) = match collision_event {
                CollisionEvent::Started(e1, e2, _) => { (*e1, *e2) },
                CollisionEvent::Stopped(_, _, _) => { continue; },
            };
    
             let (_, goal_entity) = if ball_query.get(entity1).is_ok() {
                (entity1, entity2)
            } else if ball_query.get(entity2).is_ok() {
                (entity2, entity1)
            } else {
                continue;
            };

            if let Ok(goal) = goal_query.get(goal_entity) {
                match goal {
                    Goal::Left => {
                        scoreboard.right_score += 1;

                    },
                    Goal::Right => {
                        scoreboard.left_score += 1;
                    },
                }
            }
        }
    }





