use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_vrm::VrmBundle;
use unavi_avatar::{AvatarPlugin, FallbackAvatar};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "../unavi-app/assets".to_string(),
                ..Default::default()
            }),
            PanOrbitCameraPlugin,
            AvatarPlugin,
        ))
        .add_systems(Startup, (setup_avatar, setup_scene))
        .add_systems(Update, load_avatar_two)
        .run();
}

fn setup_scene(
    mut ambient: ResMut<AmbientLight>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    ambient.brightness = 100.0;
    ambient.color = Color::rgb(0.95, 0.95, 1.0);

    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(4.5, 10.0, -7.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(10.0, 0.1, 10.0))),
        material: materials.add(StandardMaterial::default()),
        transform: Transform::from_xyz(0.0, -0.1, 0.0),
        ..default()
    });

    let mut transform = Transform::from_xyz(0.0, 3.0, -10.0);
    transform.look_at(Vec3::new(0.0, 0.5, 0.0), Vec3::new(0.0, 1.0, 0.0));

    commands.spawn((
        Camera3dBundle {
            transform,
            ..Default::default()
        },
        PanOrbitCamera::default(),
    ));
}

#[derive(Component)]
struct AvatarTwo;

fn setup_avatar(mut commands: Commands) {
    commands.spawn((
        FallbackAvatar,
        SpatialBundle {
            transform: Transform::from_xyz(-1.5, 0.0, 0.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        AvatarTwo,
        FallbackAvatar,
        SpatialBundle {
            transform: Transform::from_xyz(1.5, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

/// Simulate an avatar that takes some time to load, showing a fallback in it's place.
fn load_avatar_two(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut done: Local<bool>,
    query: Query<(Entity, &Transform), With<AvatarTwo>>,
    time: Res<Time>,
) {
    if *done || time.elapsed_seconds() < 2.0 {
        return;
    }

    *done = true;

    for (entity, transform) in query.iter() {
        commands.entity(entity).insert(VrmBundle {
            scene_bundle: SceneBundle {
                transform: *transform,
                ..Default::default()
            },
            vrm: asset_server.load("models/robot.vrm"),
            ..Default::default()
        });
    }
}
