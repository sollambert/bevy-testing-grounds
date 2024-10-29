use avian3d::prelude::{Collider, CollidingEntities};
use bevy::prelude::*;

use crate::entities::player::player::Player;

#[derive(Component, Default)]
pub struct DebugDisplay {
    pub visibility: Visibility
}

#[derive(Component)]
pub struct PlayerDebugDisplay;
#[derive(Component)]
pub struct KeyPressDebugDisplay;
#[derive(Component)]
pub struct ColliderDebugDisplay;

pub fn setup_debug_screen(
    mut commands: Commands,
) {
    commands.spawn((
        DebugDisplay {
            visibility: Visibility::Visible
        },
        NodeBundle {
            visibility: Visibility::Visible,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }
    )).with_children(|parent| {
        parent.spawn(PlayerDebugDisplay);
        parent.spawn(KeyPressDebugDisplay);
        parent.spawn(ColliderDebugDisplay);
    });
}

pub fn update_debug_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<&Player>,
    mut q_player_debug_display: Query<Entity, With<PlayerDebugDisplay>>,
    mut q_key_press_debug_display: Query<Entity, With<KeyPressDebugDisplay>>,
    mut q_collider_debug_display: Query<Entity, With<ColliderDebugDisplay>>,
    q_colliding_entities: Query<(Entity, &CollidingEntities), With<Collider>>
) {
    let player = player.single_mut();
    let player_debug_display = q_player_debug_display.single_mut();
    let key_press_debug_display = q_key_press_debug_display.single_mut();
    let collider_debug_display = q_collider_debug_display.single_mut();

    let text_style = TextStyle {
        font: asset_server.load("fonts/Roboto/Roboto-Light.ttf"),
        font_size: 16.0,
        ..default()
    };

    // Create location display
    commands.entity(player_debug_display).insert(TextBundle::from_section(
        format!("{}", player),
        text_style.to_owned()
    ));

    // Grab pressed keys and build string
    let mut keys = String::new();
    key.get_pressed().for_each(|key | {
        keys += &format!("{:?} ", key);
    });
    keys = keys.trim().to_owned();

    // Create key display
    commands.entity(key_press_debug_display).insert(TextBundle::from_section(
        format!("Keys: {}", keys),
        text_style.to_owned()
    ));

    let mut colliders_string = String::new();
    for (entity, colliding_entities) in &q_colliding_entities {
        colliders_string += &format!(
            "{:?} is colliding with: {:?}\n",
            entity,
            colliding_entities
        );
    }

    let colliders_string = colliders_string.trim();

    // Create key display
    commands.entity(collider_debug_display).insert(TextBundle::from_section(
        colliders_string,
        text_style.to_owned()
    ));
}