use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_kira_audio::{Audio as KiraAudio, AudioSource as KiraAudioSource};
use bevy_kira_audio::*;
use super::ball::Ball;


#[derive(Resource)]
pub struct GameAudioHandlers {
    pub boop: Handle<KiraAudioSource>
}

pub fn load_audio_handlers(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.insert_resource(GameAudioHandlers {boop: asset_server.load("boop.ogg")})
}

pub fn cleanup_audio_handlers(mut commands: Commands) {
    commands.remove_resource::<GameAudioHandlers>();
}

pub fn check_ball_collision(
    audio: Res<KiraAudio>,
    audio_handlers: Res<GameAudioHandlers>,
    mut collision_events: EventReader<CollisionEvent>,
    ball_query: Query<(), With<Ball>>,
) {
    for collision_event in collision_events.read() {
        let (entity1, entity2) = match collision_event {
            CollisionEvent::Started(e1, e2, _) => { (*e1, *e2) },
            CollisionEvent::Stopped(_, _, _) => { continue; },
        };

        if ball_query.get(entity1).is_ok() || ball_query.get(entity2).is_ok() {
            audio.play(audio_handlers.boop.clone());
        }
    }
}