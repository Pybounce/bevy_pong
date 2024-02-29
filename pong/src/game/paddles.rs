use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::{super::common::states::*, Ball};

#[derive(Component)]
pub struct Paddle {
    max_speed: f32,
}
impl Default for Paddle {
    fn default() -> Self {
        Self {
            max_speed: 500.0
        }
    }
}


#[derive(Component)]
pub struct PlayerPaddle {
    move_up_key: KeyCode,
    move_down_key: KeyCode
}
impl Default for PlayerPaddle {
    fn default() -> Self {
        Self { 
            move_up_key: KeyCode::KeyW,
            move_down_key: KeyCode::KeyS
        }
    }
}
#[derive(Component, Default)]
pub struct AIPaddle {

}


pub fn setup_paddles(mut commands: Commands) {
    spawn_paddle(&mut commands, true);
    spawn_paddle(&mut commands, false);
}

fn spawn_paddle(commands: &mut Commands, is_player_paddle: bool) {
    let pos: Vec3;
    if is_player_paddle {
        pos = Vec3::new(-500.0, 0.0, 0.0);
    }
    else {
        pos = Vec3::new(500.0, 0.0, 0.0);
    }
    let mut paddle = commands.spawn(SpriteBundle {
        transform: Transform {
            translation: pos,
            scale: Vec3::new(20.0, 100.0, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::rgb(2.0, 2.0, 2.0),
            ..default()
        },
        ..default()
    });
    paddle.insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0))
    .insert(GravityScale(0.0))
    .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X)
    .insert(Velocity::default())
    .insert(Paddle::default())
    .insert(DespawnOnStateExit::App(AppState::Game));
    if is_player_paddle {
        paddle.insert(PlayerPaddle::default());
    }
    else {
        paddle.insert(AIPaddle::default());
    }
}


pub fn move_paddles(
    input: Res<ButtonInput<KeyCode>>, 
    mut paddle_query: Query<(&mut Velocity, &Transform, &Paddle, Option<&PlayerPaddle>, Option<&AIPaddle>)>,
    ball_query: Query<(&Velocity, &Transform), (With<Ball>, Without<Paddle>)>
) {
    let balls: Vec<(&Velocity, &Transform)> = ball_query.iter().collect();

    for (mut v, t, paddle_data, player_control_data, ai_control_data) in &mut paddle_query {
        v.linvel = Vec2::default();

        if let Some(x) = player_control_data {
            if input.pressed(x.move_up_key) {
                v.linvel += Vec2::new(0.0, paddle_data.max_speed);
            }
            if input.pressed(x.move_down_key) {
                v.linvel -= Vec2::new(0.0, paddle_data.max_speed);
            }
        };

        

        if let Some(_) = ai_control_data {
            let mut ai_velocity_delta = Vec2::default();
            let mut current_ball_x_delta = f32::MAX;  //Distance of the current target ball from the paddle
            let mut current_ball_y_pos = f32::MAX;  //Distance of the current target ball from the paddle

            for (bv, bt) in &balls {
                let x_delta = t.translation.x - bt.translation.x;
                if (x_delta / x_delta.abs()) != (bv.linvel.x / bv.linvel.x.abs()) { continue; }
                if (current_ball_x_delta - x_delta).abs() < 5.0 && current_ball_y_pos < bt.translation.y { 
                    continue;
                }
                if x_delta > current_ball_x_delta { continue; }
                ai_velocity_delta = Vec2::default();
                current_ball_x_delta = x_delta;
                current_ball_y_pos = bt.translation.y;
                if bt.translation.y > t.translation.y + 30.0 {
                    ai_velocity_delta += Vec2::new(0.0, paddle_data.max_speed);
                }
                else if bt.translation.y < t.translation.y - 30.0 {
                    ai_velocity_delta -= Vec2::new(0.0, paddle_data.max_speed);
                }
            }
            v.linvel += ai_velocity_delta;
        }

        v.linvel = v.linvel.clamp(Vec2::new(0.0, -paddle_data.max_speed), Vec2::new(0.0, paddle_data.max_speed));

    }
}