pub mod player;
pub mod sendables;

use bevy::prelude::*;
use bevy_hookup_core::{
    hookup_component_plugin::HookupComponentPlugin, hookup_sendable_plugin::HookupSendablePlugin,
    owner_component::Owner, sync_entity::SyncEntityOwner,
};
use bevy_hookup_messenger_websocket::{
    websocket_client::WebsocketClient, websocket_client_plugin::WebsocketClientPlugin,
};
use player::{Player, PlayerPlugin};

use crate::{player::PlayerSync, sendables::Sendables};

#[tokio::main]
async fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        PlayerPlugin,
        HookupSendablePlugin::<Sendables>::default(),
        HookupComponentPlugin::<Sendables, PlayerSync>::default(),
        WebsocketClientPlugin::<Sendables>::default(),
    ))
    .insert_resource(WebsocketClient::<Sendables>::new_with_host_and_port(
        "schoggi.net".into(),
        6666,
    ))
    .insert_resource(Time::<Fixed>::from_seconds(0.05))
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 5.,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let circle = meshes.add(Circle::new(20.0));
    let color = materials.add(Color::linear_rgb(0., 1., 0.));

    for _ in 0..1000 {
        commands.spawn((
            Mesh2d(circle.clone()),
            MeshMaterial2d(color.clone()),
            SyncEntityOwner::new(),
            Transform::from_xyz(
                rand::random_range(-1000.0..1000.0),
                rand::random_range(-1000.0..1000.0),
                0.0,
            ),
            Owner::new(PlayerSync {
                ..Default::default()
            }),
            Player {
                velocity: Vec3::ZERO,
                movement_speed: rand::random_range(0.25..4.0),
            },
        ));
    }
}
