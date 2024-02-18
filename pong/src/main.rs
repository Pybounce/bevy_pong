use bevy::{
    prelude::*, 
    window::close_on_esc,
};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(PreStartup, setup_game_config)
    .add_systems(Startup, (spawn_paddles, spawn_camera))
    .add_systems(Update, close_on_esc)
    .run();
}

fn spawn_paddles(mut commands: Commands, game_config: Res<GameConfig>) {
    spawn_paddle(&mut commands, &game_config.left_paddle_config);
    spawn_paddle(&mut commands, &game_config.right_paddle_config);
}

fn spawn_paddle(commands: &mut Commands, paddle_config: &PaddleConfig) {
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: paddle_config.position.extend(0.0),
            scale: paddle_config.size.extend(1.0),
            ..Default::default()
        },
        sprite: Sprite {
            color: paddle_config.colour.into(),
            ..default()
        },
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

}




fn setup_game_config(mut commands: Commands) {
    let mut game_config = GameConfig::default();
    game_config.left_paddle_config.position.x = -500.0;
    game_config.left_paddle_config.colour = Vec4::new(0.7, 0.0, 0.0, 1.0);
    game_config.right_paddle_config.position.x = 500.0;
    game_config.right_paddle_config.colour = Vec4::new(0.0, 0.0, 0.7, 1.0);

    commands.insert_resource(game_config);
}



#[derive(Resource, Default)]
struct GameConfig
{
    left_paddle_config: PaddleConfig,
    right_paddle_config: PaddleConfig
}
struct PaddleConfig {
    colour: Vec4,
    size: Vec2,
    position: Vec2
}
impl Default for PaddleConfig {
    fn default() -> Self {
        Self {
            colour: Vec4::new(1.0, 1.0, 1.0, 1.0),
            size: Vec2::new(15.0, 100.0),
            position: Vec2::new(0.0, 0.0)
        }
    }
}