use std::fmt;

use avian3d::prelude::{AngularVelocity, Collider, CollisionLayers, Dominance, LayerMask, LinearVelocity, PhysicsLayer, RayCaster, RigidBody, SpatialQueryFilter};
use bevy::{math::*, prelude::*};

use crate::entities::EntityCollisionLayers;

pub const CAMERA_OFFSET_VEC3: Vec3 = Vec3::new(0.0, 1.75, 10.0);
pub const BODY_OFFSET_VEC3: Vec3 = Vec3::new(0.0, 1.0, 0.0);

#[derive(Component, Default)]
pub struct Player {
    pub bailed: bool,
    pub is_on_floor: bool,
    location: Vec3,
    velocity: Vec3,
    rotation: Vec3,
}

#[derive(Component)]
pub struct PlayerCamera {
    pub rotation: Vec3,
}

#[derive(Component)]
pub struct PlayerRigidBody;

#[derive(Component)]
pub struct PlayerMesh;

#[derive(Component)]
pub struct PlayerFloorRay;

#[derive(Component)]
pub struct PlayerInteractRay;

#[derive(Component)]
pub struct PlayerStepRay;

#[derive(Event)]
pub struct PlayerBailEvent(pub (Entity, bool));

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
            spawn_rotation: Option<Vec3>) -> Entity {
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
                bailed: false,
                is_on_floor: false,
                location: spawn_location,
                rotation: spawn_rotation,
                velocity: Vec3::ZERO,
            }
        )).with_children(|parent| {
            // Build child entities
            parent.spawn((
                PlayerCamera {
                    rotation: Vec3::ZERO
                },
                Camera3dBundle {
                    transform: Transform::from_translation(CAMERA_OFFSET_VEC3),
                    projection: PerspectiveProjection {
                        ..default()
                    }
                    .into(),
                    ..default()
                }
            ));
            parent.spawn((
                PlayerRigidBody,
                RigidBody::Kinematic,
                Collider::capsule(0.5, 1.0),
                CollisionLayers::new(EntityCollisionLayers::Player, [
                    EntityCollisionLayers::Ground,
                    EntityCollisionLayers::Props
                ]),
                Dominance(5),
                SpatialBundle {
                    transform: Transform::from_translation(BODY_OFFSET_VEC3),
                    ..default()
                }
            ));
            parent.spawn((
                PlayerMesh,
                PbrBundle {
                    mesh: meshes.add(Capsule3d::new(0.5, 1.0)),
                    material: materials.add(Color::srgb_u8(124, 144, 255)),
                    transform: Transform::from_translation(BODY_OFFSET_VEC3),
                    ..default()
                },
            ));
            parent.spawn((
                PlayerFloorRay,
                RayCaster::new(Vec3::new(0.0, 1.0, 0.0), Dir3::NEG_Y)
                    // .with_max_hits(2)
                    .with_max_time_of_impact(1.15)
                    .with_query_filter(
                        SpatialQueryFilter {
                            mask: LayerMask(EntityCollisionLayers::Ground.to_bits()),
                            ..default()
                        }),
            ));
            parent.spawn((
                PlayerInteractRay,
                RayCaster::new(Vec3::new(0.0, 1.0, 0.0), Dir3::NEG_Z)
                    // .with_max_hits(2)
                    .with_max_time_of_impact(2.0)
                    .with_query_filter(
                        SpatialQueryFilter {
                            mask: LayerMask(EntityCollisionLayers::Interaction.to_bits()),
                            ..default()
                        }),
            ));
        }).id()
    }
}

pub fn handle_player_bail(
    mut commands: Commands,
    mut ev_player_bail: EventReader<PlayerBailEvent>,
    mut q_player_transform: Query<(&mut Player, &mut Transform), (
        Without<PlayerCamera>, 
        Without<PlayerMesh>,
        Without<PlayerRigidBody>,
    )>,
    mut q_player_rigid_body_transform: Query<(&PlayerRigidBody, Entity, &mut Transform, &mut GlobalTransform), (
        Without<Player>, 
        Without<PlayerCamera>,
        Without<PlayerMesh>,
    )>,
    mut q_player_mesh_transform: Query<(&PlayerMesh, &mut Transform), (
        Without<Player>, 
        Without<PlayerCamera>, 
        Without<PlayerRigidBody>
    )>,
    time: Res<Time>,
) {
    let (mut player, mut _player_transform) = q_player_transform.single_mut();
    let (_player_mesh, mut player_mesh_transform) = q_player_mesh_transform.single_mut();
    let (_player_rigid_body, player_rigid_body_entity, mut player_rigid_body_transform, player_rigid_body_global_transform) = q_player_rigid_body_transform.single_mut();
    let mut player_rigid_body_entity = commands.entity(player_rigid_body_entity);
    let delta = time.delta().as_secs_f32();
    for ev in ev_player_bail.read() {
        let bailed = ev.0.1;
        player.bailed = bailed;
        if bailed {
            player_rigid_body_entity.insert(RigidBody::Dynamic);
            let current_velocity = player.get_velocity();
            println!("Player bailed! {}", current_velocity);

            // Create player rotation quaternion from rotation y value
            let body_velocity = current_velocity / delta;
            player.set_velocity(Vec3::ZERO);
            player_rigid_body_entity.insert(LinearVelocity(body_velocity));
        } else {
            println!("Player standing up!");
            player_rigid_body_entity.insert(RigidBody::Kinematic);
            player_rigid_body_entity.insert(AngularVelocity(Vec3::ZERO));
            player_rigid_body_entity.insert(LinearVelocity(Vec3::ZERO));
            player.set_location(player_rigid_body_global_transform.translation() - BODY_OFFSET_VEC3);
            *player_rigid_body_transform = Transform::from_translation(BODY_OFFSET_VEC3);
            *player_mesh_transform = Transform::from_translation(BODY_OFFSET_VEC3);
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Location: {:.4}\nYaw: {:.4}, Pitch: {:.4}, Roll: {:.4}\nVelocity: {:.4}\nBailed: {}\nOn Floor: {}",
            self.location,
            self.rotation.y.to_degrees(),
            self.rotation.x.to_degrees(),
            self.rotation.z.to_degrees(),
            self.velocity,
            self.bailed,
            self.is_on_floor)
    }
}
