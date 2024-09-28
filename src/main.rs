use bevy::app::{App, PluginGroup, Startup};
use bevy::asset::{AssetServer, Assets};
use bevy::color::Color;
use bevy::DefaultPlugins;
use bevy::math::Vec3;
use bevy::pbr::{PbrBundle, PointLightBundle, StandardMaterial};
use bevy::prelude::{Camera3dBundle, Commands, Mesh, Meshable, Plane3d, Res, ResMut, Transform};
use bevy::utils::default;
use bevy_mod_picking::{low_latency_window_plugin, DefaultPickingPlugins};
use bevy_mod_picking::highlight::DefaultHighlightingPlugin;
use arpg::player_camera::PlayerCameraPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
            PlayerCameraPlugin
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let black_material = materials.add(Color::BLACK);
    let white_material = materials.add(Color::WHITE);

    let plane_mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));

    for x in -3..4 {
        for z in -3..4 {
            commands.spawn((
                PbrBundle {
                    mesh: plane_mesh.clone(),
                    material: if (x + z) % 2 == 0 {
                        black_material.clone()
                    } else {
                        white_material.clone()
                    },
                    transform: Transform::from_xyz(x as f32 * 2.0, -1.0, z as f32 * 2.0),
                    ..default()
                },
            ));
        }
    }

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}