
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;


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
            translation: Vec3::new(0.0, 450.0, 0.0),
            scale: Vec3::new(1600.0, 10.0, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, -450.0, 0.0),
            scale: Vec3::new(1600.0, 10.0, 1.0),
            ..default()
        },
        ..default()
    })
    .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(-800.0, 0.0, 0.0),
            scale: Vec3::new(10., 900., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Goal {team: Team::Left})
    .insert(Collider::cuboid(0.5, 0.5));

    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(800.0, 0.0, 0.0),
            scale: Vec3::new(10., 900., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Goal {team: Team::Right})
    .insert(Collider::cuboid(0.5, 0.5));

    
    
    }











