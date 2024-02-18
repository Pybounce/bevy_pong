use bevy::prelude::*;
use crate::{components::velocity::Velocity, plugins::paddles::Paddle};


pub fn move_paddle(input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Paddle)>) {
    for (mut velocity, paddle) in &mut query {
        let mut new_velocity: Vec3 = Vec3::default();

        match paddle {
            Paddle::LeftPaddle => {
                if input.pressed(KeyCode::W) {
                    new_velocity += Vec3::new(0.0, 200.0, 0.0);
                }
                if input.pressed(KeyCode::S) {
                    new_velocity -= Vec3::new(0.0, 200.0, 0.0);
                }
        },
            Paddle::RightPaddle => {
                if input.pressed(KeyCode::Up) {
                    new_velocity += Vec3::new(0.0, 200.0, 0.0);
                }
                if input.pressed(KeyCode::Down) {
                    new_velocity -= Vec3::new(0.0, 200.0, 0.0);
                }
            },
        };
        velocity.current = new_velocity;
    }
}