use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use super::states::*;


pub struct PaddlesPlugin;

impl Plugin for PaddlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), (setup_paddles_config, setup_paddles).chain())
        .add_systems(OnExit(AppState::Game), cleanup_paddles_config)
        .add_systems(Update, move_paddle.run_if(in_state(GameState::UnPaused).and_then(in_state(AppState::Game))));
}
}

#[derive(Component)]
pub enum Paddle {
    RightPaddle,
    LeftPaddle
}

#[derive(Resource, Default)]
pub struct PaddlesConfig
{
    pub l_paddle: PaddleConfig,
    pub r_paddle: PaddleConfig
}

pub struct PaddleConfig {
    pub colour: Color,
    pub size: Vec2,
    pub position: Vec2,
    pub speed: i32
}

impl Default for PaddleConfig {
    fn default() -> Self {
        Self {
            colour: Color::rgb(1.0, 1.0, 1.0),
            size: Vec2::new(20.0, 100.0),
            position: Vec2::new(0.0, 0.0),
            speed: 300
        }
    }
}

fn setup_paddles(mut commands: Commands, game_config: Res<PaddlesConfig>) {
    spawn_paddle(&mut commands, &game_config.l_paddle, Paddle::LeftPaddle);
    spawn_paddle(&mut commands, &game_config.r_paddle, Paddle::RightPaddle);
}

fn spawn_paddle(commands: &mut Commands, paddle_config: &PaddleConfig, paddle_component: Paddle) {
    commands.spawn((paddle_component, SpriteBundle {
        transform: Transform {
            translation: paddle_config.position.extend(0.0),
            scale: paddle_config.size.extend(1.0),
            ..default()
        },
        sprite: Sprite {
            color: paddle_config.colour.into(),
            ..default()
        },
        ..default()
    })).insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5))
    .insert(Restitution::coefficient(1.0))
    .insert(Friction::coefficient(0.0))
    .insert(GravityScale(0.0))
    .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_X)
    .insert(Velocity::default())
    .insert(DespawnOnStateExit::App(AppState::Game));
}

fn setup_paddles_config(mut commands: Commands) {
    let mut paddles_config = PaddlesConfig::default();
    paddles_config.l_paddle.position.x = -500.0;
    paddles_config.l_paddle.colour = Color::rgb(0.9, 0.9, 0.9);
    paddles_config.r_paddle.position.x = 500.0;
    paddles_config.r_paddle.colour = Color::rgb(0.9, 0.9, 0.9);

    commands.insert_resource(paddles_config);
}

fn cleanup_paddles_config(mut commands: Commands) {
    commands.remove_resource::<PaddlesConfig>();  //TODO: See about linking resource to state
}

fn move_paddle(
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