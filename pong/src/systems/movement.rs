use bevy::prelude::*;
use crate::components::velocity::*;

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.current * time.delta_seconds();
    }
}