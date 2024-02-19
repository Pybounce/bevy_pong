use bevy::prelude::*;
use crate::{components::velocity::Velocity, plugins::paddles::*};


pub fn move_paddle(paddle_config: Res<PaddlesConfig>, input: Res<ButtonInput<KeyCode>>, mut query: Query<(&mut Velocity, &Paddle)>) {
    for (mut velocity, paddle) in &mut query {
        let mut new_velocity: Vec3 = Vec3::default();
        
        match paddle {
            Paddle::LeftPaddle => {
                if input.pressed(KeyCode::KeyW) {
                    new_velocity += Vec3::new(0.0, paddle_config.l_paddle.speed as f32, 0.0);
                }
                if input.pressed(KeyCode::KeyS) {
                    new_velocity -= Vec3::new(0.0, paddle_config.l_paddle.speed as f32, 0.0);
                }
        },
            Paddle::RightPaddle => {
                if input.pressed(KeyCode::ArrowUp) {
                    new_velocity += Vec3::new(0.0, paddle_config.l_paddle.speed as f32, 0.0);
                }
                if input.pressed(KeyCode::ArrowDown) {
                    new_velocity -= Vec3::new(0.0, paddle_config.l_paddle.speed as f32, 0.0);
                }
            },
        };
        velocity.current = new_velocity;
    }
}