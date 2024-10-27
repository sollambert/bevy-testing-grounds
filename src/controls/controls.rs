use std::process::exit;

use bevy::{prelude::*, window::{CursorGrabMode, PrimaryWindow, WindowMode}};

use crate::Game;

pub struct InputMap {
    pub left: KeyCode,
    pub right: KeyCode,
    pub back: KeyCode,
    pub forward: KeyCode,
    pub _jump: KeyCode,
    pub turn_r: KeyCode,
    pub turn_l: KeyCode,
    pub close: KeyCode,
    pub fullscreen: KeyCode,
    pub debug_menu: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        return Self {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            back: KeyCode::KeyS,
            forward: KeyCode::KeyW,
            _jump: KeyCode::Space,
            turn_r: KeyCode::ArrowRight,
            turn_l: KeyCode::ArrowLeft,
            close: KeyCode::Escape,
            fullscreen: KeyCode::F11,
            debug_menu: KeyCode::F3,
        }
    }
}

pub fn handle_key_window_functions(
    commands: Commands,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    mut game: ResMut<Game>,
    key: Res<ButtonInput<KeyCode>>
) {
    let mut primary_window = q_windows.single_mut();
    let input_map = InputMap::default();

    if key.just_pressed(input_map.close) {
        exit(0);
    }

    if key.just_pressed(input_map.debug_menu) {
        let visibility = match game.state.debug_menu_visibility {
            Visibility::Hidden => {
                Visibility::Visible
            },
            Visibility::Visible => {
                Visibility::Hidden
            },
            _ => {
                Visibility::Hidden
            }
        };
        game.state.debug_menu_visibility = visibility;
        game.debug_screen.set_visibility(commands, visibility);
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