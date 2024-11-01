use avian3d::math::PI;
use bevy::{input::*, prelude::*};
use mouse::MouseMotion;

use crate::entities::player::player::{Player, PlayerCamera, CAMERA_OFFSET_VEC3};

use super::controls::InputMap;

const MAX_WALKING_VELOCITY: f32 = 0.1;
const JUMP_VELOCITY: f32 = 20.0;
const TURN_SPEED: f32 = 1.5;
const ACCELERATION: f32 = 1.0;
const DECELERATION: f32 = 10.0;

const LEFT: Vec3 = Vec3::NEG_X;
const RIGHT: Vec3 = Vec3::X;
const FORWARD: Vec3 = Vec3::NEG_Z;
const BACKWARD: Vec3 = Vec3::Z;
const JUMP_VECTOR: Vec3 = Vec3::new(0.0, JUMP_VELOCITY, 0.0);

const MOUSE_SENSITIVITY_X: f32 = 0.003;
const MOUSE_SENSITIVITY_Y: f32 = 0.002;

const CAMERA_TOP_DEADZONE: f32 = PI / 4.0;
const CAMERA_BOTTOM_DEADZONE: f32 = PI / 4.0;

// control the game character
pub fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut player: Query<&mut Player>,
    mut player_transform: Query<&mut Transform, (With<Player>, Without<PlayerCamera>)>,
    mut player_camera_transform: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    time: Res<Time>,
) {
    let mut player = player.single_mut();
    let mut player_transform = player_transform.single_mut();
    let mut player_camera_transform = player_camera_transform.single_mut();

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
    if current_velocity.length() < MAX_WALKING_VELOCITY {
        if keyboard_input.pressed(input_map.back) {
            direction += BACKWARD;
        }
        if keyboard_input.pressed(input_map.forward) {
            direction += FORWARD;
        }
        if keyboard_input.pressed(input_map.left) {
            direction += LEFT;
        }
        if keyboard_input.pressed(input_map.right) {
            direction += RIGHT;
        }
    }

    if keyboard_input.just_pressed(input_map.jump) {
        direction += JUMP_VECTOR;
    }

    // Build rotation vector by keypress
    if keyboard_input.pressed(input_map.turn_l) {
        rotation.y += TURN_SPEED * 2.0 * PI * delta;
    }
    if keyboard_input.pressed(input_map.turn_r) {
        rotation.y -= TURN_SPEED * 2.0 * PI * delta;
    }

    // Calculate rotations from mouse deltas
    for motion in mouse_motion.read() {
        rotation.y -= motion.delta.x * MOUSE_SENSITIVITY_X * TURN_SPEED * 2.0 * PI;
        rotation.x -= motion.delta.y * MOUSE_SENSITIVITY_Y * TURN_SPEED * 2.0 * PI;
    }


    // Clamp rotation x
    rotation.x = (current_rotation.x + rotation.x).clamp(
        (-PI / 2.0  + CAMERA_TOP_DEADZONE).to_degrees(),
        (PI / 2.0 - CAMERA_BOTTOM_DEADZONE).to_degrees());

    // Add current rotation to z/y axis
    rotation.z += current_rotation.z;
    rotation.y += current_rotation.y;
    
    // Normalize rotation
    rotation %= 360.0;

    // Set current rotation
    player.set_rotation(rotation);

    // Create player rotation quaternion from rotation y value
    let rotation_quat = Quat::from_rotation_y(rotation.y.to_radians());
    
    // Get camera rotation quaternion from rotation x value
    let camera_rotation_quat = Quat::from_rotation_x(rotation.x.to_radians());

    // Multiply local direction vector by player rotation quaternion
    direction = rotation_quat.mul_vec3(direction);

    // Accelerate
    let velocity = current_velocity + (delta * ACCELERATION * direction);
    player.set_velocity(velocity);

    if direction == Vec3::ZERO {
        // Decelerate
        player.set_velocity(velocity.lerp(Vec3::ZERO, delta * DECELERATION));
    }

    // Set global position
    let global_position = player.get_location() + velocity;
    player.set_location(global_position);
    
    // Apply transforms
    *player_transform = Transform {
        translation: global_position,
        rotation: rotation_quat,
        ..default()
    };

    // Apply camera transforms
    *player_camera_transform = Transform {
        // translation: CAMERA_OFFSET_VEC3, // use for first person so camera doesn't rotate around origin
        translation: camera_rotation_quat.mul_vec3(CAMERA_OFFSET_VEC3),
        rotation: camera_rotation_quat,
        ..default()
    }
}