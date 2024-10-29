use avian3d::{prelude::{AngularVelocity, Collider, Friction, RigidBody}, PhysicsPlugins};
use bevy::prelude::*;
use controls::{controls::{handle_cursor, handle_key_window_functions}, player::move_player};
use entities::player::player::Player;
use utils::debug::{setup_debug_screen, update_debug_screen};

mod controls;
mod entities;
mod utils;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Update,move_player)
        .add_systems(Update, handle_cursor)
        .add_systems(Update, handle_key_window_functions)
        .add_systems(Update, update_debug_screen)
        .run();
}

#[derive(Resource, Default)]
struct Game;

fn setup(
    mut commands: Commands,
    mut _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    setup_debug_screen(commands.reborrow());
    Player::spawn(commands.reborrow(), 
        meshes.reborrow(),
        materials.reborrow(),
        Some(Vec3::new(0.0, 0.0, 5.0)),
        None);
    
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(20.0, 0.1),
        Friction::new(0.5),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(4.0, 0.1)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            ..default()
        },
    ));

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}