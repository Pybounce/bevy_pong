use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_kira_audio::{Audio as KiraAudio, AudioPlugin as KiraAudioPlugin, AudioSource as KiraAudioSource};
use bevy_kira_audio::*;
use super::ball::Ball;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
        fn build(&self, app: &mut App) {
        app.add_plugins(KiraAudioPlugin)
        .add_systems(Startup, load_audio_handles)
        .add_systems(Update, check_ball_collision);
    }
}
#[derive(Resource)]
struct AudioHandlers {
    boop: Handle<KiraAudioSource>
}

fn load_audio_handles(asset_server: Res<AssetServer>,mut commands: Commands) {
    commands.insert_resource(AudioHandlers {boop: asset_server.load("boop.ogg")})
}

fn check_ball_collision(
    audio: Res<KiraAudio>,
    audio_handlers: Res<AudioHandlers>,
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