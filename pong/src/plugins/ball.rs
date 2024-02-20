use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BALL_SPEED: f32 = 1000.0;
const BALL_SIZE: Vec2 = Vec2::new(20.0, 20.0);

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
        .add_systems(Update, clamp_velocity);
    }
}

#[derive(Component)]
pub struct Ball;


fn spawn_ball(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::default(),
            scale: BALL_SIZE.extend(1.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(0.0, 1.0, 0.0),
            ..default()
        },
        ..default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0))
    .insert(Velocity::linear(Vec2::new(3.0, 0.0)))
    .insert(GravityScale(0.0))
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(Ball);
}

fn clamp_velocity(mut query: Query<&mut Velocity, With<Ball>>) {
    for mut velocity in &mut query {
        velocity.linvel = velocity.linvel.normalize() * BALL_SPEED;
    }
}
