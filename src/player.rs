use crate::actions::Actions;
use crate::loading::{Animations, SceneAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use std::time::Duration;
use bevy::animation::{animate_targets, RepeatAnimation};
use crate::interpolators::{InterpolatingComponent, InterpolationFactor};
use crate::vstransform::VSTransform;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_walker)
            .add_systems(Update, sync_animation_speed.run_if(in_state(GameState::Playing)));
            /*add_systems(Update, move_player.run_if(in_state(GameState::Playing)));*/
    }
}

fn spawn_walker(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        brightness: 300.0,
    });

    let start_transform = Transform {
        translation: Vec3::new(-1.8, -1.0, -5.0),
        rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), // 90 degrees rotation around Y-axis
        ..default()
    };

    let end_transform = Transform {
        translation: Vec3::new(1.8, -1.0, -5.0),
        rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), // 90 degrees rotation around Y-axis
        ..default()
    };


    commands.spawn((
        SceneBundle {
            scene: scene_assets.walker.clone(),
            transform: Transform {
                translation: default(),
                rotation: Quat::from_rotation_y(std::f32::consts::FRAC_PI_2), // 90 degrees rotation around Y-axis
                ..default()
            },
            ..default()
        },
    ))
        .insert(InterpolatingComponent::<VSTransform>::standard(start_transform.into(), end_transform.into()));
}

// System to update the AnimationPlayer's speed based on InterpolatingComponent<f32>
fn sync_animation_speed(
    mut query: Query<(&mut AnimationPlayer)>,
    time: ResMut<Time>,
) {
    for (mut player) in query.iter_mut() {
        for (_, playing_animation) in player.playing_animations_mut() {
            let new_speed = time.elapsed_seconds().cos();
            playing_animation.set_speed(new_speed);
        }
    }
}