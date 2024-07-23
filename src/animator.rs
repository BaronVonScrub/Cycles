//! Plays animations from a skinned glTF.

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use bevy::{
    prelude::*,
};
use bevy::asset::AssetContainer;
use bevy::ecs::observer::TriggerTargets;
use crate::GameState;
use crate::loading::AnimationAssets;

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    #[allow(dead_code)]
    graph: Handle<AnimationGraph>,
}

// System to play animation
fn play_animations(
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    animation_clips: Res<Assets<AnimationClip>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    // Check if the animation clip is loaded
    if let Some(walker_clip) = animation_clips.get(&animations.walker_walk) {
        // Create a new animation graph
        let mut graph = AnimationGraph::new();
        let animation_node = graph.add_clip(animations.walker_walk.clone(), 1.0, graph.root);

        // Add the graph to the asset server and get its handle
        let graph_handle = graphs.add(graph);

        // Loop through each player and play the animation
        for (entity, mut player) in players.iter_mut() {
            // Use AnimationTransitions to manage the animation
            let mut transitions = AnimationTransitions::new();
            transitions.play(&mut player, animation_node, Duration::ZERO).repeat();

            commands.entity(entity)
                .insert(graph_handle.clone())
                .insert(transitions);
        }
    }
}
pub struct EzAnimationPlugin;

impl Plugin for EzAnimationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, play_animations.run_if(in_state(GameState::Playing)));
    }
}