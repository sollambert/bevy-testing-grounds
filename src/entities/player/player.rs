use std::fmt;

use avian3d::prelude::{Collider, RigidBody};
use bevy::{math::*, prelude::*};

pub const CAMERA_OFFSET_VEC3: Vec3 = Vec3::new(0.0, 1.75, 10.0);

pub struct Player {
    entity: Option<Entity>,
    camera: Option<Entity>,
    rigid_body: Option<Entity>,
    mesh: Option<Entity>,
    location: Vec3,
    velocity: Vec3,
    rotation: Vec3,
    camera_bundle: Camera3dBundle,
    rigid_body_bundle: (RigidBody, Collider),
}

impl Player {
    pub fn get_entity(&self) -> Entity {
        self.entity.unwrap_or_else(|| {
            panic!("Attempted to access player entity without spawning bundle!")
        })
    }
    pub fn get_camera(&self) -> Entity {
        self.camera.unwrap_or_else(|| {
            panic!("Attempted to access player entity without spawning bundle!")
        })
    }
    pub fn get_rigid_body(&self) -> Entity {
        self.rigid_body.unwrap_or_else(|| {
            panic!("Attempted to access player entity without spawning bundle!")
        })
    }
    pub fn get_mesh(&self) -> Entity {
        self.mesh.unwrap_or_else(|| {
            panic!("Attempted to access player entity without spawning bundle!")
        })
    }
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
    pub fn spawn_bundle(&mut self,
            mut commands: Commands,
            mut meshes: Mut<Assets<Mesh>>,
            mut materials: Mut<Assets<StandardMaterial>>,
            spawn_location: Option<Vec3>,
            spawn_rotation: Option<Vec3>) {
        // Create rotation for starting position
        let spawn_rotation = spawn_rotation.unwrap_or(Vec3::ZERO);

        // Create transform for starting position
        let transform = Transform {
            translation: spawn_location.unwrap_or(Vec3::ZERO),
            rotation: Quat::from_euler(EulerRot::XYZ,
                spawn_rotation.x,
                spawn_rotation.y,
                spawn_rotation.z),
            ..default()
        };

        // Build spatial bundle for base entity
        self.entity = Some(commands.spawn(
            SpatialBundle {
                transform,
                ..default()
            }).id());

        // Build child entities
        self.camera = Some(commands.spawn(self.camera_bundle.clone()).id());
        self.rigid_body = Some(commands.spawn(self.rigid_body_bundle.clone()).id());
        self.mesh = Some(commands.spawn(
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
                material: materials.add(Color::srgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
        }).id());

        // Add entities as children to player object
        commands.entity(self.entity.unwrap())
            .push_children(
                &[self.get_rigid_body(),
                self.get_camera(),
                self.get_mesh()]);
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location: {:.4}\nRotation: {:.4}\nVelocity: {:.4}", self.location, self.rotation, self.velocity)
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            entity: None,
            camera: None,
            rigid_body: None,
            mesh: None,
            location: Vec3::ZERO,
            rotation: Vec3::ZERO,
            velocity: Vec3::ZERO,
            rigid_body_bundle: (
                RigidBody::Kinematic,
                Collider::capsule(0.5, 1.0)),
            camera_bundle: 
                    Camera3dBundle {
                        transform: Transform::from_translation(CAMERA_OFFSET_VEC3),
                        projection: PerspectiveProjection {
                            // fov: 70.0_f32.to_radians(),
                            ..default()
                        }
                        .into(),
                        ..default()
                    },
        }
    }
}
