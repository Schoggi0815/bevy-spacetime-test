use bevy::prelude::*;
use crossbeam::channel::{unbounded, Receiver};
use spacetimedb_sdk::{credentials, DbContext, Error, Identity, Table, TableWithPrimaryKey};

use crate::{
    module_bindings::{set_position, DbConnection, ErrorContext, PlayerTableAccess},
    player::{OtherPlayer, Player},
};

pub struct SpaceTimeConnectionPlugin;

impl Plugin for SpaceTimeConnectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, (check_event_system, send_player_position));
    }
}

#[derive(Resource)]
struct ServerConnection(DbConnection);

const HOST: &str = "http://localhost:3000";

const DB_NAME: &str = "test-server";

#[derive(Resource)]
struct PlayerEvents {
    receiver: Receiver<PlayerUpdateEvent>,
}

struct PlayerUpdateEvent {
    position: Vec3,
    player_id: Identity,
}

fn startup(mut commands: Commands) {
    let token = creds_store().load().expect("Error loading credentials");
    let connection = DbConnection::builder()
        // Register our `on_connect` callback, which will save our auth token.
        .on_connect(on_connected)
        // Register our `on_connect_error` callback, which will print a message, then exit the process.
        .on_connect_error(on_connect_error)
        // Our `on_disconnect` callback, which will print a message, then exit the process.
        .on_disconnect(on_disconnected)
        // If the user has previously connected, we'll have saved a token in the `on_connect` callback.
        // In that case, we'll load it and pass it to `with_token`,
        // so we can re-authenticate as the same `Identity`.
        .with_token(token.clone())
        // Set the database name we chose when we called `spacetime publish`.
        .with_module_name(DB_NAME)
        // Set the URI of the SpacetimeDB host that's running our database.
        .with_uri(HOST)
        // Finalize configuration and connect!
        .build()
        .expect("Failed to connect");

    let (sender, receiver) = unbounded::<PlayerUpdateEvent>();
    let sender2 = sender.clone();

    connection.db.player().on_insert(move |_ctx, player| {
        if _ctx.identity() == player.identity {
            return;
        }
        sender
            .try_send(PlayerUpdateEvent {
                player_id: player.identity,
                position: Vec3::new(player.position_x, player.position_y, player.position_z),
            })
            .expect("unbounded channel should never block!");
    });

    connection.db.player().on_update(move |_ctx, _old, player| {
        if _ctx.identity() == player.identity {
            return;
        }
        sender2
            .try_send(PlayerUpdateEvent {
                player_id: player.identity,
                position: Vec3::new(player.position_x, player.position_y, player.position_z),
            })
            .expect("unbounded channel should never block!");
    });

    connection
        .subscription_builder()
        .on_error(|_ctx, err| {
            error!("{}", err);
        })
        .subscribe("SELECT * FROM player WHERE online = true");

    connection.run_threaded();

    commands.insert_resource(PlayerEvents { receiver });

    commands.insert_resource(ServerConnection(connection));
}

fn send_player_position(
    player_query: Query<&Transform, With<Player>>,
    server_connection: Res<ServerConnection>,
) {
    let player = player_query.single();

    server_connection
        .0
        .reducers
        .set_position(
            player.translation.x,
            player.translation.y,
            player.translation.z,
        )
        .expect("Couldn't send player pos");
}

fn check_event_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut other_player_qeury: Query<(&mut Transform, &OtherPlayer)>,
    player_events: Res<PlayerEvents>,
) {
    for event in player_events.receiver.try_iter() {
        let other_player = other_player_qeury
            .iter_mut()
            .find(|(_, other_player)| other_player.0 == event.player_id);

        if let Some(mut other_player) = other_player {
            other_player.0.translation = event.position;
        } else {
            let circle = meshes.add(Circle::new(20.0));
            let color = Color::linear_rgb(1., 0., 0.);

            commands.spawn((
                OtherPlayer(event.player_id),
                Transform::from_translation(event.position),
                Mesh2d(circle),
                MeshMaterial2d(materials.add(color)),
            ));
        }
    }
}

fn creds_store() -> credentials::File {
    credentials::File::new("player-1")
}

fn on_connected(_ctx: &DbConnection, _identity: Identity, token: &str) {
    if let Err(e) = creds_store().save(token) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

/// Our `on_connect_error` callback: print the error, then exit the process.
fn on_connect_error(_ctx: &ErrorContext, err: Error) {
    eprintln!("Connection error: {:?}", err);
    std::process::exit(1);
}

/// Our `on_disconnect` callback: print a note, then exit the process.
fn on_disconnected(_ctx: &ErrorContext, err: Option<Error>) {
    if let Some(err) = err {
        eprintln!("Disconnected: {}", err);
        std::process::exit(1);
    } else {
        println!("Disconnected.");
        std::process::exit(0);
    }
}
