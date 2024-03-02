use bevy::prelude::*;




pub struct TweenPlugin;
impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tween_positions);
    }
}


#[derive(Component)]
pub struct PositionTween {
    start_time: f32,
    duration: f32,
    velocity: Vec3,
}

impl PositionTween {
    pub fn new(start_time: f32, duration: f32, offset: Vec3) -> Self {
        let speed = offset.length() / duration;
        let mut vel: Vec3 = offset.normalize() * speed;
        if offset.length() <= 0.0 { vel = Vec3::default(); }

        return Self {
            start_time,
            duration,
            velocity: vel
        }
    }
}

fn tween_positions(
    mut query: Query<(&mut Transform, &PositionTween, Entity)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (mut tranform, tween_data, e) in &mut query {
        let lerp_t = (time.elapsed_seconds() - tween_data.start_time) / tween_data.duration;
        if lerp_t <= 0.0 { continue; }    //start time not reached yet
        if lerp_t > 1.0 { commands.entity(e).remove::<PositionTween>(); }   //remove component?

        tranform.translation += tween_data.velocity * time.delta_seconds();
    }
}


