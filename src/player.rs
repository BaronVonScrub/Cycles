use crate::actions::Actions;
use crate::loading::{Animations, SceneAssets, TextureAssets};
use crate::GameState;
use bevy::prelude::*;
use std::time::Duration;
use bevy::animation::{animate_targets, RepeatAnimation};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(OnEnter(GameState::Playing), spawn_walker);
            /*dd_systems(Update, move_player.run_if(in_state(GameState::Playing)));*/
    }
}

fn spawn_walker(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: scene_assets.walker.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -1.0, -5.0)),
            ..default()
        },
    ));
}