use bevy::prelude::*;




pub struct TweenPlugin;
impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tween_positions);
    }
}


#[derive(Component)]
pub struct PositionTween {
    pub start_time: f32,
    pub duration: f32,
    pub start_pos: Vec3,
    pub target_pos: Vec3,
}

fn tween_positions(
    mut query: Query<(&mut Transform, &PositionTween, Entity)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (mut tranform, tween_data, e) in &mut query {
        let lerp_t = (time.elapsed_seconds() - tween_data.start_time) / tween_data.duration;
        if lerp_t <= 0.0 { continue; }    //start time not reached yet

        let offset = (tween_data.target_pos - tween_data.start_pos) * lerp_t;
        tranform.translation = tween_data.start_pos + offset;

        if lerp_t >= 1.0 { 
            tranform.translation = tween_data.target_pos;   //this doesn't work with multiple!!!
            commands.entity(e).remove::<PositionTween>(); 
        }
    }
}


