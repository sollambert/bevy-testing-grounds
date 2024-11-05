use avian3d::{prelude::{AngularVelocity, Collider, CollisionLayers, Friction, LayerMask, PhysicsDebugPlugin, RigidBody}, PhysicsPlugins};
use bevy::{prelude::*, render::mesh::ConeMeshBuilder};
use controls::{controls::{handle_cursor, handle_debug_keys, handle_key_window_functions}, player::{handle_player_camera, handle_bailed_player_movement, handle_player_is_on_floor, handle_player_movement}};
use entities::{player::player::{handle_player_bail, Player, PlayerBailEvent}, EntityCollisionLayers};
use utils::debug::{setup_debug_screen, update_debug_screen};

mod controls;
mod entities;
mod utils;

fn main() {
    let plugins = (DefaultPlugins,
        PhysicsPlugins::default());
    let mut app = App::new();
    app.add_plugins(plugins);
    if cfg!(debug_assertions) {
        let debug_plugins = PhysicsDebugPlugin::default();
        app.add_plugins(debug_plugins)
            .add_systems(Startup, setup_debug_screen)
            .add_systems(Update, handle_debug_keys)
            .add_systems(Update, update_debug_screen);
    }
    app.init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_event::<PlayerBailEvent>()
        .add_systems(Update,handle_player_is_on_floor)
        .add_systems(Update,handle_player_camera)
        .add_systems(Update,handle_player_movement)
        .add_systems(Update,handle_bailed_player_movement)
        .add_systems(Update, handle_player_bail)
        .add_systems(Update, handle_cursor)
        .add_systems(Update, handle_key_window_functions)
        .run();
}

#[derive(Resource)]
struct Game {
    dev_mode: bool
}

impl Default for Game {
    fn default() -> Game {
        let mut dev_mode = false;
        if cfg!(debug_assertions) {
            dev_mode = true;
        }
        Game {
            dev_mode
        }
    }
}

fn setup(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    Player::spawn(commands.reborrow(), 
        meshes.reborrow(),
        materials.reborrow(),
        Some(Vec3::new(0.0, 0.25, 5.0)),
        None);

    // // spawn generator
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/Generator.glb")),
    //     ..default()
    // });
    
    // Static physics object with a collision shape
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(200.0, 0.1),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        Friction::new(0.5),
        PbrBundle {
            mesh: meshes.add(Cylinder::new(200.0, 0.1)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, -0.05, 0.0),
            ..default()
        },
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 10.0, 10.0),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        PbrBundle {
            mesh: meshes.add(Cuboid::new(10.0, 10.0, 10.0)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 5.0, -20.0),
            ..default()
        },
    ));
    commands.spawn((
        RigidBody::Static,
        Collider::cone(10.0, 1.0),
        CollisionLayers::new(EntityCollisionLayers::Ground, LayerMask::ALL),
        PbrBundle {
            mesh: meshes.add(ConeMeshBuilder::new(10.0, 1.0, 16)),
            material: materials.add(Color::srgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(20.0, 0.5, -20.0),
            ..default()
        },
    ));

    // Dynamic physics object with a collision shape and initial angular velocity
    for _i in 0..10 {
        commands.spawn((
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            CollisionLayers::new(EntityCollisionLayers::Props, LayerMask::ALL),
            AngularVelocity(Vec3::new(2.5, 3.5, 1.5)),
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb_u8(124, 144, 255)),
                transform: Transform::from_xyz(0.0, 4.0, 0.0),
                ..default()
            },
        ));
    }

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