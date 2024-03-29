
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ball::Ball;
use super::scoreboard::Scoreboard;
use super::super::common::states::*;

pub const LEVEL_AREA: Vec2 = Vec2::new(1400.0, 700.0);

#[derive(Component)]
pub enum Goal {
    Left,
    Right
}

pub fn setup_level(mut commands: Commands) {

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
    .insert(DespawnOnStateExit::App(AppState::Game));

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
    .insert(DespawnOnStateExit::App(AppState::Game));

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
    .insert(DespawnOnStateExit::App(AppState::Game));

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
    .insert(DespawnOnStateExit::App(AppState::Game));

    
    //background-inside
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec2::default().extend(-100.0),
            scale: Vec3::new(LEVEL_AREA.x, LEVEL_AREA.y, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::Game));
    //background-outside (border)
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec2::default().extend(-110.0),
            scale: Vec3::new(LEVEL_AREA.x + 5.0, LEVEL_AREA.y + 5.0, 1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(2.5, 2.5, 2.5),
            ..default()
        },
        ..default()
    })
    .insert(DespawnOnStateExit::App(AppState::Game));



}

pub fn check_goal_collision(
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<(), With<Ball>>,
    goal_query: Query<&Goal>,
    mut scoreboard: ResMut<Scoreboard>,
    mut game_state: ResMut<NextState<GameState>>
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
                game_state.set(GameState::Resetting);
            }
        }
    }





