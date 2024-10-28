use avian3d::prelude::CollidingEntities;
use bevy::prelude::*;

use crate::{entities::player::player::Player, Game};

pub struct DebugScreen {
    entity: Option<Entity>,
    location_section: Option<Entity>,
    key_section: Option<Entity>,
    colliders_section: Option<Entity>,
}

impl DebugScreen {
    pub fn build(&mut self, mut commands: Commands, visibility: Visibility) {
        self.entity = Some(commands.spawn(NodeBundle {
            visibility,
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).id());
        self.colliders_section = None;
        self.key_section = None;
        self.location_section =  None;
    }
    pub fn set_visibility(&mut self, mut commands: Commands, visibility: Visibility) {
        commands.entity(self.entity.unwrap()).insert(NodeBundle {
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

impl Default for DebugScreen {
    fn default() -> Self {
        Self {
            entity: None,
            location_section: None,
            key_section: None,
            colliders_section: None,
        }
    }
}

pub fn update_debug_screen(
    mut commands: Commands,
    mut game: ResMut<Game>,
    asset_server: Res<AssetServer>,
    key: Res<ButtonInput<KeyCode>>,
    mut player: Query<&Player>,
    query: Query<(Entity, &CollidingEntities)>) {
    let player = player.single_mut();

    let style = TextStyle {
        font: asset_server.load("fonts/Roboto/Roboto-Light.ttf"),
        font_size: 16.0,
        ..default()
    };

    // Create location display
    if let Some(location_section) = game.debug_screen.location_section {
        commands.entity(location_section).insert(TextBundle::from_section(
            format!("{}", player),
            style.to_owned()
        ));
    } else {
        game.debug_screen.location_section = Some(commands.spawn(TextBundle::from_section(
            format!("Position: {}", player),
            style.to_owned()
        )).id());
    }

    // Grab pressed keys and build string
    let mut keys = String::new();
    key.get_pressed().for_each(|key | {
        keys += &format!("{:?} ", key);
    });
    keys = keys.trim().to_owned();

    // Create key display
    if let Some(key_section) = game.debug_screen.key_section {
        commands.entity(key_section).insert(TextBundle::from_section(
            format!("Keys: {}", keys),
            style.to_owned()
        ));
    } else {
        game.debug_screen.key_section = Some(commands.spawn(TextBundle::from_section(
            format!("Keys: {}", keys),
            style.to_owned()
        )).id());
    }

    let mut colliders_string = String::new();
    for (entity, colliding_entities) in &query {
        colliders_string += &format!(
            "{:?} is colliding with the following entities: {:?}\n",
            entity,
            colliding_entities
        );
    }

    let colliders_string = colliders_string.trim();

    // Create key display
    if let Some(colliders_section) = game.debug_screen.colliders_section {
        commands.entity(colliders_section).insert(TextBundle::from_section(
            colliders_string,
            style.to_owned()
        ));
    } else {
        game.debug_screen.colliders_section = Some(commands.spawn(TextBundle::from_section(
            colliders_string,
            style.to_owned()
        )).id());
    }

    commands.entity(game.debug_screen.entity.unwrap()).push_children(&[
        game.debug_screen.location_section.unwrap(),
        game.debug_screen.key_section.unwrap(),
        game.debug_screen.colliders_section.unwrap()
    ]);
}

// pub fn print_keys() {
//     let mut keys = String::new();
//     key.get_pressed().for_each(|key | {
//         keys += &format!("{:?} ", key);
//     });
//     keys = keys.trim().to_owned();
//     println!("{}", keys);
// }

// pub fn log_location(game: ResMut<Game>) {
//     println!("{}", game.player);
// }

// pub fn report_colliders() {
//     for (entity, colliding_entities) in &query {
//         println!(
//             "{:?} is colliding with the following entities: {:?}",
//             entity,
//             colliding_entities
//         );
//     }
// }