
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const LEVEL_AREA: Vec2 = Vec2::new(1400.0, 700.0);

pub enum Team {
    Left,
    Right
}

#[derive(Component)]
pub struct Goal {
    pub team: Team
}


pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level);
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
    .insert(Goal {team: Team::Left})
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
    .insert(Goal {team: Team::Right})
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











