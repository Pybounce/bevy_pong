use bevy::prelude::*;
use crate::plugins::paddles::*;
use bevy_rapier2d::prelude::*;


pub fn move_paddle(
    paddle_config: Res<PaddlesConfig>, 
    input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut Velocity, &Paddle)>
) 
{
    for (mut velocity, paddle) in &mut query {
        let mut new_velocity: Vec2 = Vec2::default();
        
        match paddle {
            Paddle::LeftPaddle => {
                if input.pressed(KeyCode::KeyW) {
                    new_velocity += Vec2::new(0.0, paddle_config.l_paddle.speed as f32);
                }
                if input.pressed(KeyCode::KeyS) {
                    new_velocity -= Vec2::new(0.0, paddle_config.l_paddle.speed as f32);
                }
        },
            Paddle::RightPaddle => {
                if input.pressed(KeyCode::ArrowUp) {
                    new_velocity += Vec2::new(0.0, paddle_config.l_paddle.speed as f32);
                }
                if input.pressed(KeyCode::ArrowDown) {
                    new_velocity -= Vec2::new(0.0, paddle_config.l_paddle.speed as f32);
                }
            },
        };
        velocity.linvel  = new_velocity;
    }
}