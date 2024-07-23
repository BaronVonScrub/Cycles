use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use bevy::color::Color::Oklcha;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use crate::GameState;
use crate::vstransform::VSTransform;
// Implement InterpolatableValue for Transform

#[derive(Reflect, InspectorOptions)]
pub struct Curve<T: VectorSpace + Clone + Send + Sync + 'static>(CubicCurve<T>);

impl<T: VectorSpace + VectorSpace + Clone + Send + Sync + 'static> Curve<T> {
    pub fn new(tension: f32, keyframes: Vec<T>) -> Self {
        Curve(CubicCardinalSpline::new(tension, keyframes).to_curve())
    }

    pub fn sample(&self, t: f32) -> T {
        let segments = self.0.segments.len();
        let t = segments as f32 * t;
        self.0.position(t)
    }
}

// Define the InterpolatableComponent struct
#[derive(Reflect, Component, InspectorOptions)]
#[reflect(Component)]
struct KeyframingComponent<T: VectorSpace + Clone + Send + Sync + 'static> {
    curve: Curve<T>,
    current: T,
}

impl<T: VectorSpace + Clone + Send + Sync + 'static> KeyframingComponent<T> {
    pub fn new(tension: f32, keyframes: impl Into<Vec<T>>) -> Self {
        let keyframes_vec: Vec<T> = keyframes.into();
        KeyframingComponent {
            curve: Curve::new(tension, keyframes_vec.clone()),
            current: keyframes_vec.first().unwrap().clone(),
        }
    }

    pub fn interpolate(&mut self, t: f32) {
        self.current = self.curve.sample(t);
    }
}

// Define a resource to store the interpolation factor
#[derive(Resource)]
struct InterpolationFactor(f32);

// System to interpolate all InterpolatableComponent instances
fn keyframe_system<T: VectorSpace + Clone + Send + Sync + 'static>(
    mut query: Query<&mut KeyframingComponent<T>>,
    interpolation_factor: Res<InterpolationFactor>,
) {
    for mut component in query.iter_mut() {
        component.interpolate(interpolation_factor.0);
    }
}

// EXAMPLE
fn _spawn_cube_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
        commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        brightness: 1000.0,
    });

    let start_transform = Transform {
        translation: Vec3::new(-1.0, 0.0, -3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(1.0, 1.0, 1.0), // Adjust the scale as needed
    };

    let end_transform = Transform {
        translation: Vec3::new(1.0, 0.0, -3.0),
        rotation: Quat::IDENTITY,
        scale: Vec3::new(2.0, 2.0, 2.0), // Adjust the scale as needed
    };

    let start_color: Oklaba = bevy_color::Oklcha::new(0.7, 0.1257, 218.46, 1.0).into();
    let mid_color: Oklaba = bevy_color::Oklcha::new(0.7, 0.1257, 106.29, 1.0).into();
    let end_color: Oklaba = bevy_color::Oklcha::new(0.7, 0.1257, 10.15, 1.0).into();

    let init_color = start_color.clone();

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::from(init_color),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: material_handle.clone(),
        transform: Transform{
            translation: Vec3::new(-1.0, 0.0, -3.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::new(1.0, 1.0, 1.0), // Adjust the scale as needed
        },
        ..Default::default()
    })
        .insert(KeyframingComponent::new(0.0,[start_color,mid_color, end_color]))
        .insert(MaterialHandle(material_handle));
}

// System to update LocalTransform based on the lerpd value
fn update_local_transform_system(
    mut query: Query<(&mut Transform, &KeyframingComponent<VSTransform>)>,
) {
    for (mut transform, interpolating_component) in query.iter_mut() {
        *transform = interpolating_component.current.0;
    }
}

fn update_color_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &KeyframingComponent<Oklaba>, &Handle<StandardMaterial>)>,
) {
    for (entity, color_component, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            material.base_color = color_component.current.into();
        } else {
            // In case the material is not found, create a new material and update the entity's handle
            let new_material = materials.add(StandardMaterial {
                base_color: color_component.current.into(),
                ..Default::default()
            });
            commands.entity(entity).insert(new_material);
        }
    }
}

fn update_interpolation_factor_system(
    time: Res<Time>,
    mut interpolation_factor: ResMut<InterpolationFactor>,
) {
    interpolation_factor.0 = (time.elapsed_seconds()).sin() * 0.5 + 0.5;
}

#[derive(Component)]
struct MaterialHandle(Handle<StandardMaterial>);

// Define the InterpolationPlugin
pub struct EzKeyframingPlugin;

impl Plugin for EzKeyframingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InterpolationFactor(0.0))
            .register_type::<KeyframingComponent<VSTransform>>()
            .register_type::<KeyframingComponent<Oklaba>>()
            .register_type::<KeyframingComponent<f32>>()
            .add_systems(Update, update_interpolation_factor_system.run_if(in_state(GameState::Playing)))
            .add_systems(Update,
                         (
                             keyframe_system::<VSTransform>,
                             keyframe_system::<Oklaba>,
                             keyframe_system::<f32>,
                         ).run_if(in_state(GameState::Playing)))
            .add_systems(Update,
                         (
                             //update_local_transform_system,
                             update_color_system
                         )
                             .run_if(in_state(GameState::Playing)));;
    }
}
