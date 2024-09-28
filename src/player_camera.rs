use bevy::app::{App, Plugin, Startup};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::{default, on_event, Camera3dBundle, Commands, Component, Entity, Event, EventReader, IntoSystemConfigs, Mesh, Meshable, Plane3d, Query, ResMut, Resource, Transform, Update, Vec3, With, Without};
use bevy_mod_picking::PickableBundle;
use bevy_mod_picking::prelude::{ListenerInput, On, Pointer, Up};

#[derive(Component, Clone)]
pub struct Player;

#[derive(Component, Clone)]
pub struct PlayerCamera {
    pub offset: Vec3,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            offset: 2.0*Vec3::new(-1.0, 2.0, -1.0),
        }
    }
}

#[derive(Event)]
struct SetDestination(Entity, f32, f32);

impl From<ListenerInput<Pointer<Up>>> for SetDestination {
    fn from(event: ListenerInput<Pointer<Up>>) -> Self {
        let pos = event.hit.position.unwrap_or(Vec3::ZERO);
        SetDestination(event.target, pos.x, pos.z)
    }
}

fn handle_set_player_destination(
    mut event_reader: EventReader<SetDestination>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    event_reader.read().for_each(|set_destination| {
        let SetDestination(entity, x, z) = set_destination;
        let dst = Vec3::new(*x, 0.0, *z);
        if let Ok(mut transform) = player_query.get_mut(entity.clone()) {
            transform.translation = dst;
        }
    });
}

fn setup_player_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let red_material = materials.add(Color::linear_rgb(1.0, 0.0, 0.0));
    let plane_mesh = meshes.add(Plane3d::default().mesh().size(3.0, 3.0));

    let player_camera = PlayerCamera::default();
    let player = Player;
    let player_entity = commands.spawn_empty().id();
    if let Some(mut entity_commands) = commands.get_entity(player_entity) {
        entity_commands
            .insert(player)
            .insert(
                PbrBundle {
                    mesh: plane_mesh.clone(),
                    material: red_material,
                    transform: Transform::from_translation(Vec3::ZERO),
                    ..default()
                }
            )
            .insert(PickableBundle::default())
            .insert(On::<Pointer<Up>>::send_event::<SetDestination>());
        ;
    }
    commands.spawn(
        (
            player_camera.clone(),
            Camera3dBundle {
                transform: Transform::from_translation(player_camera.offset)
                    .looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            }
        )
    );
}

fn update_player_camera(
    mut player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &PlayerCamera), Without<Player>>,
) {
    let results = (player_query.get_single(), camera_query.get_single_mut());
    if let (Ok(player_transform), Ok((mut camera_transform, player_camera))) = results {
        camera_transform.translation = player_transform.translation + player_camera.offset;
        camera_transform.look_at(player_transform.translation, Vec3::Y);
    }
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SetDestination>()
            .add_systems(Startup, setup_player_camera)
            .add_systems(
                Update,
                (
                    handle_set_player_destination.run_if(on_event::<SetDestination>()),
                    update_player_camera
                ),
            )
        ;
    }
}