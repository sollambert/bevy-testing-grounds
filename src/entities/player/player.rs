use std::fmt;

use avian3d::prelude::{Collider, RigidBody};
use bevy::{math::*, prelude::*};

pub const CAMERA_OFFSET_VEC3: Vec3 = Vec3::new(0.0, 1.75, 10.0);


#[derive(Component)]
pub struct Player {
    location: Vec3,
    velocity: Vec3,
    rotation: Vec3,
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerRigidBody;

#[derive(Component)]
pub struct PlayerMesh;

impl Player {
    pub fn get_location(&self) -> Vec3 {
        return self.location;
    }
    pub fn get_rotation(&self) -> Vec3 {
        return self.rotation;
    }
    pub fn get_velocity(&self) -> Vec3 {
        return self.velocity;
    }
    pub fn set_location(&mut self, new_location: Vec3) {
        self.location = new_location;
    }
    pub fn set_rotation(&mut self, new_rotation: Vec3) {
        self.rotation = new_rotation;
    }
    pub fn set_velocity(&mut self, new_velocity: Vec3) {
        self.velocity = new_velocity
    }
    pub fn spawn(mut commands: Commands,
            mut meshes: Mut<Assets<Mesh>>,
            mut materials: Mut<Assets<StandardMaterial>>,
            spawn_location: Option<Vec3>,
            spawn_rotation: Option<Vec3>) {
        // Create rotation for starting position
        let spawn_rotation = spawn_rotation.unwrap_or(Vec3::ZERO);
        let spawn_location = spawn_location.unwrap_or(Vec3::ZERO);

        // Create transform for starting position
        let transform = Transform {
            translation: spawn_location,
            rotation: Quat::from_euler(EulerRot::XYZ,
                spawn_rotation.x,
                spawn_rotation.y,
                spawn_rotation.z),
            ..default()
        };

        // Build player entity
        commands.spawn((
            SpatialBundle {
                transform,
                ..default()
            },
            Player {
                location: spawn_location,
                rotation: spawn_rotation,
                velocity: Vec3::ZERO,
            }
        )).with_children(|parent| {
            // Build child entities
            parent.spawn((
                PlayerCamera,
                Camera3dBundle {
                    transform: Transform::from_translation(CAMERA_OFFSET_VEC3),
                    projection: PerspectiveProjection {
                        // fov: 70.0_f32.to_radians(),
                        ..default()
                    }
                    .into(),
                    ..default()
                }
            ));
            parent.spawn((
                PlayerRigidBody,
                RigidBody::Kinematic,
                Collider::capsule(0.5, 1.0)
            ));
            parent.spawn((
                PlayerMesh,
                PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
                    material: materials.add(Color::srgb_u8(124, 144, 255)),
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..default()
                }
            ));
        });
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location: {:.4}\nRotation: {:.4}\nVelocity: {:.4}", self.location, self.rotation, self.velocity)
    }
}
