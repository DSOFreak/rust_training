use std::thread::yield_now;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::transform::commands;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(FixedUpdate, spawn_particles)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    for i in 0..100 {
        // Circle
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(10.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            Object {
                velocity: Vec3::new(rand::random::<f32>() * 100.0, rand::random::<f32>() * 100.0, 0.0),
                gravity: 0.0,
                damping: 0.01,
                ttl: 2.0,
            },
        ));
    }

}

#[derive(Component)]
struct Object {
    velocity: Vec3,
    ttl: f32,
    gravity: f32,
    damping: f32,
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.

fn sprite_movement(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut Object)>) {
    for (entity, mut transform, mut object) in query.iter_mut() {
        object.ttl -= time.delta_seconds();
        if object.ttl <= 0.0 {
            commands.entity(entity).despawn();
        }
        object.velocity.y -= object.gravity * time.delta_seconds();
        let damping = object.damping;
        object.velocity *= 1.0 - damping * time.delta_seconds();
        transform.translation += object.velocity * time.delta_seconds();
    }
}

const PARTICLE_SPAWN_RATE: f32 = 2.0;
const PARTICLE_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
fn spawn_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let dist = 200.0;
    let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
    let x = dist * angle.cos();
    let y = dist * angle.sin();
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::RED)),
            transform: Transform::from_translation(PARTICLE_SPAWN_POSITION),
            ..default()
        },
        Object {
            velocity: Vec3::new(x, y, 0.0),
            gravity: 0.0,
            damping: 0.01,
            ttl: 2.0,
        },
    ));
}