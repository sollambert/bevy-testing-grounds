use std::process::exit;

use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowMode}};

use crate::{entities::player::player::{Player, PlayerBailEvent}, utils::debug::DebugDisplay};

pub struct InputMap {
    pub left: KeyCode,
    pub right: KeyCode,
    pub back: KeyCode,
    pub forward: KeyCode,
    pub jump: KeyCode,
    pub turn_r: KeyCode,
    pub turn_l: KeyCode,
    pub close: KeyCode,
    pub fullscreen: KeyCode,
    pub debug_bail: KeyCode,
    pub debug_menu: KeyCode,
    pub debug_reset_position: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        return Self {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            back: KeyCode::KeyS,
            forward: KeyCode::KeyW,
            jump: KeyCode::Space,
            turn_r: KeyCode::ArrowRight,
            turn_l: KeyCode::ArrowLeft,
            close: KeyCode::Escape,
            fullscreen: KeyCode::F11,

            // debug keys
            debug_bail: KeyCode::KeyB,
            debug_menu: KeyCode::F3,
            debug_reset_position: KeyCode::KeyR,
        }
    }
}

pub fn handle_key_window_functions(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut primary_window = q_windows.single_mut();
    let input_map = InputMap::default();

    if key.just_pressed(input_map.close) {
        exit(0);
    }

    if key.just_pressed(input_map.fullscreen) {
        match primary_window.mode {
            WindowMode::Windowed => {
                primary_window.mode = WindowMode::BorderlessFullscreen;
            },
            WindowMode::BorderlessFullscreen => {
                primary_window.mode = WindowMode::Windowed;
            },
            _ => {
                primary_window.mode = WindowMode::Windowed;
            }
        }
    }
}

pub fn handle_debug_keys(
    mut commands: Commands,
    key: Res<ButtonInput<KeyCode>>,
    mut ev_player_bail: EventWriter<PlayerBailEvent>,
    mut q_debug_menu: Query<(Entity, &mut DebugDisplay)>,
    mut q_player: Query<(Entity, &mut Player)>,
) {
    let input_map = InputMap::default();
    let (player_entity, mut player) = q_player.single_mut();
    let (debug_menu_entity, mut debug_display) = q_debug_menu.single_mut();

    if key.just_pressed(input_map.debug_bail) {
        ev_player_bail.send(PlayerBailEvent((player_entity, !player.bailed)));
    }

    if key.just_pressed(input_map.debug_reset_position) {
        player.set_location(Vec3::ZERO);
    }

    if key.just_pressed(input_map.debug_menu) {
        let mut visibility: Visibility = Visibility::Visible;
        let mut debug_menu_commands = commands.entity(debug_menu_entity);
        if debug_display.visibility == Visibility::Visible {
            visibility = Visibility::Hidden;
        }
        debug_display.visibility = visibility;
        debug_menu_commands.insert(NodeBundle {
            visibility,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        });
    }
}

pub fn handle_cursor(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let mut primary_window = q_windows.single_mut();
    
        // if you want to use the cursor, but not let it leave the window,
        // use `Confined` mode:
        primary_window.cursor.grab_mode = CursorGrabMode::Confined;
    
        // for a game that doesn't use the cursor (like a shooter):
        // use `Locked` mode to keep the cursor in one place
        primary_window.cursor.grab_mode = CursorGrabMode::Locked;
    
        // also hide the cursor
        primary_window.cursor.visible = false;
    }

    if key.just_pressed(KeyCode::AltLeft) {
        let mut primary_window = q_windows.single_mut();
    
        primary_window.cursor.grab_mode = CursorGrabMode::None;
        primary_window.cursor.visible = true;
    }
}