use std::{thread, time::Duration};

use avian3d::{math::{PI, TAU}, prelude::{RayCaster, RayHits}};
use bevy::{input::*, math::VectorSpace, prelude::*};
use mouse::MouseMotion;

use crate::entities::player::player::{Player, PlayerBody, PlayerCamera, PlayerCameraRay, PlayerFloorRay, BODY_OFFSET_VEC3, CAMERA_OFFSET_VEC3, CAMERA_RAY_OFFSET_VEC3};

use super::controls::InputMap;

// const LEFT: Vec3 = Vec3::NEG_X;
// const RIGHT: Vec3 = Vec3::X;
const FORWARD: Vec3 = Vec3::NEG_Z;
const BACKWARD: Vec3 = Vec3::Z;
const GRAVITY: Vec3 = Vec3::new(0.0, -6., 0.0);
const JUMP_VELOCITY: Vec3 = Vec3::new(0.0, 0.5, 0.0);

const MAX_WALKING_VELOCITY: f32 = 1.0;
const TURN_SPEED: f32 = PI / 16.0;
const ACCELERATION: f32 = 0.3;
const DECELERATION: f32 = 1.0;

const MOUSE_SENSITIVITY_X: f32 = 0.003;
const MOUSE_SENSITIVITY_Y: f32 = 0.002;

const CAMERA_TOP_DEADZONE: f32 = PI / 4.0;
const CAMERA_BOTTOM_DEADZONE: f32 = PI / 4.0;

const LEAN_SPEED: f32 = 2.0;
const LEFT_LEAN_MAX_ANGLE: f32 = PI / 8.0;
const RIGHT_LEAN_MAX_ANGLE: f32 = -PI / 8.0;

pub fn handle_player_is_on_floor(
    mut q_player: Query<&mut Player>,
    mut q_player_floor_ray: Query<(&RayCaster, &RayHits), With<PlayerFloorRay>>,
) {
    let mut player = q_player.single_mut();
    let current_velocity = player.get_velocity();
    let mut current_location = player.get_location();
    for (player_floor_caster, player_floor_hits) in q_player_floor_ray.iter_mut() {
        for floor_hit in player_floor_hits.iter() {
            let max_time_of_impact = player_floor_caster.max_time_of_impact;
            player.is_on_floor = true;
            if current_velocity.y < 0.0 {
                player.set_velocity(current_velocity * Vec3::new(1.0, 0.0, 1.0));
            }
            if (max_time_of_impact - floor_hit.time_of_impact).abs() > 0.01 {
                current_location.y += max_time_of_impact - floor_hit.time_of_impact;
                player.set_location(current_location);
            }
            return;
        }
    }
    player.is_on_floor = false;
}

// control the game character
pub fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Player, &mut Transform)>,
    mut q_player_body_transform: Query<(&mut PlayerBody, Entity, &mut Transform, &mut GlobalTransform), (
        Without<Player>, 
        Without<PlayerCamera>,
    )>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = q_player.single_mut();
    if player.bailed {
        return;
    }
    let (mut player_body, player_body_entity, mut player_body_transform, player_body_global_transform) = q_player_body_transform.single_mut();

    // Create delta from 
    let delta = time.delta().as_secs_f32();
    let current_velocity = player.get_velocity();
    let current_rotation = player.get_rotation();
    let mut current_lean = player_body.lean;

    // Initialize vectors
    let mut direction = Vec3::ZERO;
    let mut rotation = Vec3::ZERO;

    // Create default input map
    let input_map = InputMap::default();

    // Build direction vector by keypress
    if current_velocity.length() < MAX_WALKING_VELOCITY && player.is_on_floor  {
        if keyboard_input.pressed(input_map.back) {
            direction += BACKWARD;
        }
        if keyboard_input.pressed(input_map.forward) {
            direction += FORWARD;
        }
    }

    // Turn player
    if keyboard_input.pressed(input_map.left) {
        rotation.y += TURN_SPEED * TAU * delta;
        current_lean = current_lean.lerp(LEFT_LEAN_MAX_ANGLE, LEAN_SPEED * delta);
    }
    else if keyboard_input.pressed(input_map.right) {
        rotation.y -= TURN_SPEED * TAU * delta;
        current_lean = current_lean.lerp(RIGHT_LEAN_MAX_ANGLE, LEAN_SPEED * delta);
    } else {
        current_lean = current_lean.lerp(0.0, LEAN_SPEED * delta);
    }

    player_body.lean = current_lean;

    // Add current rotation to z/y axis
    rotation.z += current_rotation.z;
    rotation.y += current_rotation.y;
    
    // Normalize rotation
    rotation %= TAU;

    // Set current rotation
    player.set_rotation(rotation);

    // Create player rotation quaternion from rotation y value
    let rotation_quat = Quat::from_rotation_y(rotation.y);

    let player_body_rotation_quat = Quat::from_rotation_z(player_body.lean);

    // Multiply local direction vector by player rotation quaternion
    direction = rotation_quat.mul_vec3(direction);
    if !player.is_on_floor {
        direction += GRAVITY;
    }

    // Accelerate
    let mut velocity = current_velocity + (delta * ACCELERATION * direction);

    if player.is_on_floor {
        if keyboard_input.just_pressed(input_map.jump) {
            velocity += JUMP_VELOCITY;
        } else {
            velocity = velocity.lerp(Vec3::ZERO, delta * DECELERATION);
        }
    }
    
    player.set_velocity(velocity);

    // Set global position
    let global_position = player.get_location() + velocity;
    player.set_location(global_position);
    
    // Apply transforms
    *player_transform = Transform {
        translation: global_position,
        rotation: rotation_quat,
        ..default()
    };
    *player_body_transform = Transform {
        translation: BODY_OFFSET_VEC3,
        rotation: player_body_rotation_quat,
        ..default()
    }
}

pub fn handle_player_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_player_transform: Query<(&mut Player, &mut Transform), (
        Without<PlayerBody>,
        Without<PlayerCamera>, 
        Without<PlayerCameraRay>,
    )>,
    mut q_player_body_transform: Query<(&PlayerBody, &mut Transform), (
        Without<Player>, 
        Without<PlayerCamera>,
        Without<PlayerCameraRay>,
    )>,
    mut q_player_camera_transform: Query<(&mut PlayerCamera, &mut Transform, &mut GlobalTransform), (
        Without<Player>,
        Without<PlayerBody>,
        Without<PlayerCameraRay>,
    )>,
    mut q_player_camera_ray: Query<(&mut RayCaster, Option<&RayHits>),(
        With<PlayerCameraRay>,
        Without<Player>,
        Without<PlayerBody>,
        Without<PlayerCamera>,
    )>,
    time: Res<Time>,
) {
    let (player, _player_transform) = q_player_transform.single_mut();
    let (mut player_camera, mut player_camera_transform, mut player_camera_global_transform) = q_player_camera_transform.single_mut();
    let (_player_body, player_body_transform) = q_player_body_transform.single_mut();
    let (mut player_camera_caster, player_camera_hits) = q_player_camera_ray.single_mut();

    let delta = time.delta().as_secs_f32();

    let mut rotation = Vec3::ZERO;
    let current_rotation = player_camera.rotation;

    // Calculate rotations from mouse deltas
    for motion in mouse_motion.read() {
        rotation.y -= motion.delta.x * MOUSE_SENSITIVITY_X * TAU * delta;
        rotation.x -= motion.delta.y * MOUSE_SENSITIVITY_Y * TAU * delta;
    }

    // Clamp rotation x
    rotation.x = (current_rotation.x + rotation.x).clamp(
        -PI / 2.0  + CAMERA_TOP_DEADZONE,
        PI / 2.0 - CAMERA_BOTTOM_DEADZONE
    );

    // Add current rotation to z/y axis
    rotation.z += current_rotation.z;
    rotation.y += current_rotation.y;
    
    // Normalize rotation
    rotation %= TAU;

    // Set current rotation
    player_camera.rotation = rotation;

    let mut camera_offset = CAMERA_OFFSET_VEC3;
    
    // Get camera rotation quaternion from rotation x value
    let camera_rotation_quat = Quat::from_euler(EulerRot::YXZ, rotation.y, rotation.x, 0.0);


    // Check if camera is colliding 
    if let Some(player_camera_hits) = player_camera_hits {
        if let Some(camera_hit) = player_camera_hits.iter().next() {
            camera_offset = camera_offset.lerp(CAMERA_RAY_OFFSET_VEC3, 1.0 - ((camera_hit.time_of_impact - 1.0) / player_camera_caster.max_time_of_impact));
        }
    }

    let camera_offset_rotation_applied = camera_rotation_quat.mul_vec3(camera_offset);
    let default_camera_offset_with_rotation = camera_rotation_quat.mul_vec3(CAMERA_OFFSET_VEC3);

    println!("Camera offset: {}", camera_offset_rotation_applied);

    if player.bailed {
        // let camera_offset_rotation_applied = camera_rotation_quat.mul_vec3(camera_offset);
        // let default_camera_offset_with_rotation = camera_rotation_quat.mul_vec3(CAMERA_OFFSET_VEC3);
        // Apply camera transforms
        *player_camera_transform = Transform {
            translation: player_body_transform.translation
                + camera_offset_rotation_applied
                - BODY_OFFSET_VEC3,
            rotation: camera_rotation_quat,
            ..default()
        };
        // Apply camera ray transforms
        player_camera_caster.origin = player_body_transform.translation
            + CAMERA_RAY_OFFSET_VEC3
            - BODY_OFFSET_VEC3;
        player_camera_caster.direction = Dir3::from_xyz(
            default_camera_offset_with_rotation.x, 
            default_camera_offset_with_rotation.y - CAMERA_RAY_OFFSET_VEC3.y,
            default_camera_offset_with_rotation.z
        ).unwrap();
    } else {
        // Apply camera transforms
        *player_camera_transform = Transform {
            // translation: CAMERA_OFFSET_VEC3, // use for first person so camera doesn't rotate around origin
            translation: camera_offset_rotation_applied,
            rotation: camera_rotation_quat,
            ..default()
        };
        // Apply camera ray transforms
        player_camera_caster.origin = CAMERA_RAY_OFFSET_VEC3;
        player_camera_caster.direction = Dir3::from_xyz(
            default_camera_offset_with_rotation.x, 
            default_camera_offset_with_rotation.y - CAMERA_RAY_OFFSET_VEC3.y,
            default_camera_offset_with_rotation.z
        ).unwrap();
    }
}

pub fn handle_bailed_player_movement(
    mut q_player_transform: Query<(&mut Player, &mut Transform), (
        Without<PlayerCamera>, 
        Without<PlayerBody>,
    )>,
    mut q_player_body_transform: Query<(&PlayerBody, &mut Transform), (
        Without<Player>, 
        Without<PlayerCamera>,
    )>,
) {
    let (player, _player_transform) = q_player_transform.single_mut();
    if !player.bailed {
        return;
    }
    let (_player_body, _player_body_transform) = q_player_body_transform.single_mut();
    // todo!("Implement minor movement adjustments when player is bailed.")
}