use bevy::prelude::*;


//TODO: This entire tweening should work with multiple tweens of the same kind, for one entity - it currently doesn't.
//Check if having Entity and commands ruins parallelism, if it does, move the removal of Tween Components into one system


pub struct TweenPlugin;
impl Plugin for TweenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (tween_positions, tween_colours));
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
    for (mut transform, tween_data, e) in &mut query {
        let lerp_t = (time.elapsed_seconds() - tween_data.start_time) / tween_data.duration;
        if lerp_t <= 0.0 { continue; }    //start time not reached yet

        let offset = (tween_data.target_pos - tween_data.start_pos) * lerp_t;
        transform.translation = tween_data.start_pos + offset;

        if lerp_t >= 1.0 { 
            transform.translation = tween_data.target_pos;   //this doesn't work with multiple!!!
            commands.entity(e).remove::<PositionTween>(); 
        }
    }
}


#[derive(Component)]
pub struct ColorTween {
    pub start_time: f32,
    pub duration: f32,
    pub start_color: Color,
    pub target_color: Color
}

fn tween_colours(
    mut query: Query<(&mut Sprite, &ColorTween, Entity)>,
    time: Res<Time>,
    mut commands: Commands
) {
    for (mut sprite, tween_data, e) in &mut query {
        let mut lerp_t = (time.elapsed_seconds() - tween_data.start_time) / tween_data.duration;
        lerp_t *= lerp_t * lerp_t;
        if lerp_t <= 0.0 { continue; }    //start time not reached yet

        let color_rgba = ((tween_data.target_color.rgba_to_vec4() - tween_data.start_color.rgba_to_vec4()) * lerp_t) + tween_data.start_color.rgba_to_vec4();
        sprite.color = Color::rgba(color_rgba.x, color_rgba.y, color_rgba.z, color_rgba.w);

        if lerp_t >= 1.0 { 
            sprite.color = tween_data.target_color;   //this doesn't work with multiple!!!
            commands.entity(e).remove::<ColorTween>(); 
        }
    }
}


