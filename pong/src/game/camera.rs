
use bevy::prelude::*;
use super::Ball;

pub fn update_camera_position(mut camera_transform_query: Query<&mut Transform, (With<Camera>, Without<Ball>)>, ball_transform_query: Query<&Transform, With<Ball>>) {
    let mut camera_transform = match camera_transform_query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return
    };

    let ball_transform = match ball_transform_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return
    };

    camera_transform.translation.x = ball_transform.translation.x / 30.0;
    camera_transform.translation.y = ball_transform.translation.y / 10.0;
}

pub fn reset_camera_position(mut query: Query<&mut Transform, With<Camera>>) {
    let mut transform = match query.get_single_mut() {
        Ok(transform) => transform,
        Err(_) => return
    };
    transform.translation = Vec3::default();
}