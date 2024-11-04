use avian3d::{math::{PI, TAU}, prelude::{RayCaster, RayHits}};
use bevy::{input::*, prelude::*};
use mouse::MouseMotion;

use crate::entities::player::player::{Player, PlayerCamera, PlayerFloorRay, PlayerBody, BODY_OFFSET_VEC3, CAMERA_OFFSET_VEC3};

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

const RIGHT_LEAN_MAX_ANGLE: f32 = PI / 4.0;
const LEFT_LEAN_MAX_ANGLE: f32 = -(PI / 4.0);

pub fn handle_player_is_on_floor(
    mut q_player: Query<&mut Player>,
    q_player_floor_ray: Query<(&RayCaster, &RayHits), With<PlayerFloorRay>>,
) {
    let mut player = q_player.single_mut();
    let current_velocity = player.get_velocity();
    let mut current_location = player.get_location();
    for (player_floor_caster, player_floor_hits) in q_player_floor_ray.iter() {
        for floor_hit in player_floor_hits.iter() {
            player.is_on_floor = true;
            if current_velocity.y < 0.0 {
                player.set_velocity(current_velocity * Vec3::new(1.0, 0.0, 1.0));
            }
            current_location.y += player_floor_caster.max_time_of_impact - floor_hit.time_of_impact - 0.15;
            player.set_location(current_location);
            return;
        }
    }
    player.is_on_floor = false;
}

// control the game character
pub fn handle_player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_player: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = q_player.single_mut();
    if player.bailed {
        return;
    }

    // Create delta from 
    let delta = time.delta().as_secs_f32();
    let current_velocity = player.get_velocity();
    let current_rotation = player.get_rotation();

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
    }
    if keyboard_input.pressed(input_map.right) {
        rotation.y -= TURN_SPEED * TAU * delta;
    }

    // Add current rotation to z/y axis
    rotation.z += current_rotation.z;
    rotation.y += current_rotation.y;
    
    // Normalize rotation
    rotation %= TAU;

    // Set current rotation
    player.set_rotation(rotation);

    // Create player rotation quaternion from rotation y value
    let rotation_quat = Quat::from_rotation_y(rotation.y);

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
}

pub fn handle_player_camera(
    mut mouse_motion: EventReader<MouseMotion>,
    mut q_player_transform: Query<(&mut Player, &mut Transform), (
        Without<PlayerCamera>, 
        Without<PlayerBody>,
    )>,
    mut q_player_body_transform: Query<(&PlayerBody, &mut Transform), (
        Without<Player>, 
        Without<PlayerCamera>,
    )>,
    mut q_player_camera_transform: Query<(&mut PlayerCamera, &mut Transform), (
        Without<Player>,
        Without<PlayerBody>,
    )>,
    time: Res<Time>,
) {
    let (player, _player_transform) = q_player_transform.single_mut();
    let (mut player_camera, mut player_camera_transform) = q_player_camera_transform.single_mut();
    let (_player_body, player_body_transform) = q_player_body_transform.single_mut();

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
    
    // Get camera rotation quaternion from rotation x value
    let camera_rotation_quat = Quat::from_euler(EulerRot::YXZ, rotation.y, rotation.x, 0.0);

    if player.bailed {
        // Apply camera transforms
        *player_camera_transform = Transform {
            translation: camera_rotation_quat.mul_vec3(
                CAMERA_OFFSET_VEC3
                + player_body_transform.translation
                - BODY_OFFSET_VEC3
            ),
            rotation: camera_rotation_quat,
            ..default()
        };
    } else {
        // Apply camera transforms
        *player_camera_transform = Transform {
            // translation: CAMERA_OFFSET_VEC3, // use for first person so camera doesn't rotate around origin
            translation: camera_rotation_quat.mul_vec3(CAMERA_OFFSET_VEC3),
            rotation: camera_rotation_quat,
            ..default()
        }
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