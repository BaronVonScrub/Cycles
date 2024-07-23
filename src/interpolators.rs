use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use crate::GameState;
use crate::vstransform::VSTransform;

// Define the elerp method for Vec3
fn elerp(v1: Vec3, v2: Vec3, t: f32) -> Vec3 {
    let x_factor_log = (1. - t) * v1.x.log2() + t * v2.x.log2();
    let y_factor_log = (1. - t) * v1.y.log2() + t * v2.y.log2();
    let z_factor_log = (1. - t) * v1.z.log2() + t * v2.z.log2();

    Vec3::new(
        x_factor_log.exp2(),
        y_factor_log.exp2(),
        z_factor_log.exp2(),
    )
}

#[derive(Reflect, InspectorOptions)]
pub struct Curve(CubicCurve<f32>);

impl Default for Curve {
    fn default() -> Self {
        Curve(CubicCardinalSpline::new(0.5, [0.0f32, 1.0f32]).to_curve()) // Assuming ForeignType has a `new` method
    }
}

// Define the InterpolatableComponent struct
#[derive(Reflect, Component, Default, InspectorOptions)]
#[reflect(Component)]
pub struct InterpolatingComponent<T: VectorSpace + Clone + Send + Sync + 'static> {
    start: T,
    end: T,
    current: T,
    curve: Curve
}

impl<T: VectorSpace + Clone + Send + Sync + 'static> InterpolatingComponent<T> {
    pub fn standard(start: T, end: T) -> Self {
        let curr = start.clone();
        InterpolatingComponent {
            start,
            end,
            current: curr,
            curve: Default::default(),
        }
    }

    pub fn curved(start: T, end: T, points: impl Into<Vec<f32>>) -> Self {
        let curr = start.clone();
        InterpolatingComponent {
            start,
            end,
            current: curr,
            curve: Curve(CubicCardinalSpline::new(0.5, points).to_curve())
        }
    }

    pub fn lerp(&mut self, t: f32) {
        let segments =self.curve.0.segments.len();
        let t= self.curve.0.position(segments as f32*t);
        self.current = self.start.lerp(self.end, t);
    }
}

// Define a resource to store the interpolation factor
#[derive(Resource)]
struct InterpolationFactor(f32);

// System to lerp all InterpolatableComponent instances
fn lerp_system<T: VectorSpace + Clone + Send + Sync + 'static>(
    mut query: Query<&mut InterpolatingComponent<T>>,
    interpolation_factor: Res<InterpolationFactor>,
) {
    for mut component in query.iter_mut() {
        component.lerp(interpolation_factor.0);
    }
}

// System to update the interpolation factor based on the sine of the game time
fn update_interpolation_factor_system(
    time: Res<Time>,
    mut interpolation_factor: ResMut<InterpolationFactor>,
) {
    interpolation_factor.0 = (time.elapsed_seconds()).sin() * 0.5 + 0.5;
}

// EXAMPLE System to spawn a cube with an InterpolatableComponent wrapping a Transform and Oklaba color
/*
fn spawn_cube_system(
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

    let start_color = Oklaba::new(0.3, 0.5, 0.0, 1.0);
    let end_color = Oklaba::new(1.0, 0.5, 1.0, 1.0);

    let init_color = start_color.clone();

    let material_handle = materials.add(StandardMaterial {
        base_color: Color::from(init_color),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: material_handle.clone(),
        transform: start_transform,
        ..Default::default()
    })
        .insert(InterpolatingComponent::<VSTransform>::curved(start_transform.into(), end_transform.into(), [0.0,0.2,1.0]))
        .insert(InterpolatingComponent::standard(start_color, end_color))
        .insert(MaterialHandle(material_handle));
}*/

#[derive(Component)]
struct MaterialHandle(Handle<StandardMaterial>);

// System to update LocalTransform based on the lerpd value
fn update_local_transform_system(
    mut query: Query<(&mut Transform, &InterpolatingComponent<VSTransform>)>,
) {
    for (mut transform, interpolating_component) in query.iter_mut() {
        *transform = interpolating_component.current.0;
    }
}

fn update_color_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &InterpolatingComponent<Oklaba>, &Handle<StandardMaterial>)>,
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

// Define the InterpolationPlugin
pub struct EzAnimationPlugin;

impl Plugin for EzAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InterpolationFactor(0.0))
            .register_type::<InterpolatingComponent<VSTransform>>()
            .register_type::<InterpolatingComponent<Oklaba>>()
            .register_type::<InterpolatingComponent<f32>>()
            //.add_systems(OnEnter(GameState::Playing),spawn_cube_system)
            .add_systems(Update, update_interpolation_factor_system.run_if(in_state(GameState::Playing)))
            .add_systems(Update,
                         (
                             lerp_system::<VSTransform>,
                             lerp_system::<Oklaba>,
                             lerp_system::<f32>,
                         ).run_if(in_state(GameState::Playing)))
            .add_systems(Update,
                         (
                             update_local_transform_system,
                             update_color_system
                         )
                .run_if(in_state(GameState::Playing)));
    }
}